// use self::utils::construct_ed25519_instruction;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::ed25519_program;
// use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::{keccak, secp256k1_recover::secp256k1_recover};
use libsecp256k1;

declare_id!("3FURtoJPkZABZXjhsFrnxMQqXh2oJUkKdUkHkpLssCa2");

pub mod utils;

#[program]
pub mod sig_verify {

    use super::*;

    pub fn signature_verify(
        _ctx: Context<SignatureVerify>,
        _args: SignatureVerifyArgs,
    ) -> Result<()> {
        // let verify_ed25519_instruction =
        //     construct_ed25519_instruction(args.pubkey, args.signature, args.msg);

        // invoke(
        //     &verify_ed25519_instruction,
        //     &vec![ctx.accounts.ed25519_program.to_account_info()],
        // )?;

        msg!("Verify across!");

        Ok(())
    }

    pub fn secp256k1_recover_instruction(
        _ctx: Context<Secp256k1RecoverInstruction>,
        args: Secp256k1RecoverInstructionArgs,
    ) -> Result<()> {
        let message_hash = {
            let mut hasher = keccak::Hasher::default();
            hasher.hash(&args.message);
            hasher.result()
        };

        {
            let signature = libsecp256k1::Signature::parse_standard_slice(&args.signature)
                .map_err(|_| ProgramError::InvalidArgument)
                .unwrap();

            if signature.s.is_high() {
                msg!("signature with high-s value");
            }
        }

        let recovered_pubkey =
            secp256k1_recover(&message_hash.0, args.recovery_id, &args.signature)
                .map_err(|_| ProgramError::InvalidArgument)?;

        require!(
            recovered_pubkey.0 == args.public_key,
            SigVeriryError::InvalidPublicKey
        );

        Ok(())
    }
}

#[error_code]
pub enum SigVeriryError {
    #[msg("Publick key is invalid!")]
    InvalidPublicKey,
}

#[derive(Accounts)]
pub struct Secp256k1RecoverInstruction<'info> {
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Secp256k1RecoverInstructionArgs {
    pub public_key: [u8; 64],
    pub message: Vec<u8>,
    pub signature: [u8; 64],
    pub recovery_id: u8,
}

#[derive(Accounts)]
pub struct SignatureVerify<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub ed25519_program: Program<'info, Ed25519>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct SignatureVerifyArgs {
    pub pubkey: Pubkey,
    pub signature: [u8; 64],
    pub msg: String,
}

pub struct Ed25519;

impl anchor_lang::Id for Ed25519 {
    fn id() -> Pubkey {
        ed25519_program::ID
    }
}
