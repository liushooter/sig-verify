import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SigVerify } from "../target/types/sig_verify";
import { keccak256 } from "ethereum-cryptography/keccak.js"
import * as ethUtil from "@ethereumjs/util";
import nacl from "tweetnacl";

describe("sig-verify", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  anchor.setProvider(provider);

  const program = anchor.workspace.SigVerify as Program<SigVerify>;

  const msg = Uint8Array.from(Buffer.from('this is such a good message to sign'));

  const person = anchor.web3.Keypair.generate();
  const fakePerson = anchor.web3.Keypair.generate();

  let signature: Uint8Array;

  before(async () => {

    let txid = await provider.connection.requestAirdrop(
      person.publicKey,
      5 * anchor.web3.LAMPORTS_PER_SOL,
    );

    let blockhash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      signature: txid,
      blockhash: blockhash.blockhash,
      lastValidBlockHeight: blockhash.lastValidBlockHeight
    });

    signature = nacl.sign.detached(msg, person.secretKey);
  })

  it("Ethereum recovery", async () => {
    const privateKey = ethUtil.hexToBytes("0x1111111111111111111111111111111111111111111111111111111111111111")
    const publicKey = ethUtil.privateToPublic(privateKey)

    const messageForActivation = new TextEncoder().encode("DePIN") // activation msg
    const hashedMessageForActivation = keccak256(messageForActivation)

    const { r, s, v} = ethUtil.ecsign(hashedMessageForActivation, privateKey)
    const signature = Uint8Array.from([...r, ...s])
    const recoveryId = Number(ethUtil.calculateSigRecovery(v))

    // const ethAddress = anchor.web3.Secp256k1Program.publicKeyToEthAddress(pubKey);
    const tx = await program.methods
      .secp256K1RecoverInstruction({
        publicKey: publicKey,
        message: Buffer.from(messageForActivation),
        signature: signature,
        recoveryId: recoveryId,
      })
      .rpc()
  });
});
