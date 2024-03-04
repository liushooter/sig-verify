import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SigVerify } from "../target/types/sig_verify";
import nacl from "tweetnacl";
import * as secp from "@noble/secp256k1";

describe("sig-verify", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  anchor.setProvider(provider);

  const program = anchor.workspace.SigVerify as Program<SigVerify>;

  const MSG = Uint8Array.from(Buffer.from('this is such a good message to sign'));

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

    signature = nacl.sign.detached(MSG, person.secretKey);
  })

  it("Signature verify", async () => {
    // secp256k1
    const privKey = secp.utils.randomPrivateKey();
    const pubKeyRaw = secp.getPublicKey(privKey);
    const msg = Buffer.from('hello world');
    const msgHash = await secp.utils.sha256(msg);
    const signatureSecp = await secp.sign(msgHash, privKey, { recovered: true });
    const isValid = secp.verify(signatureSecp[0], msgHash, pubKeyRaw);
    const pubKey = pubKeyRaw.slice(1);
    console.log("isValid: ", isValid)
    console.log("length: ", pubKey.length);

    // const ethAddress = anchor.web3.Secp256k1Program.publicKeyToEthAddress(pubKey);
    const ins = await program.methods
      .secp256K1RecoverInstruction({
        publicKey: Array.from(pubKey),
        message: Buffer.from(msg),
        signature: Array.from(signatureSecp[0]),
        recoveryId: signatureSecp[1],
      })
      .instruction()

    let tx = new anchor.web3.Transaction();
    tx.add(ins);
    const txsig = await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      tx,
      [person]
    )

    // const tx = await program.methods
    //   .signatureVerify({
    //     pubkey: person.publicKey,
    //     signature: Array.from(signature),
    //     msg: MSG.toString(),
    //   })
    //   .accounts({
    //     payer: person.publicKey,
    //     ed25519Program: anchor.web3.Ed25519Program.programId,
    //   })
    //   .signers([person])
    //   .rpc();

    // let tx = new anchor.web3.Transaction();
    // tx.add(
    //   // Ed25519 instruction
    //   anchor.web3.Ed25519Program.createInstructionWithPublicKey({
    //     publicKey: person.publicKey.toBytes(),
    //     // publicKey: fakePerson.publicKey.toBytes(),
    //     message: MSG,
    //     signature: signature,
    //   })
    // )

    // const txsig = await anchor.web3.sendAndConfirmTransaction(
    //   provider.connection,
    //   tx,
    //   [person],
    // )

    // const tx = await program.methods
    //   .secp256k1RecoverInstruction({
    //     message: Array.from(MSG),

    //   })

    console.log("Transaction signature is :", txsig);
  });
});


