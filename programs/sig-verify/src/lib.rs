use self::utils::construct_ed25519_instruction;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::ed25519_program;
use anchor_lang::solana_program::program::invoke;

declare_id!("3FURtoJPkZABZXjhsFrnxMQqXh2oJUkKdUkHkpLssCa2");

pub mod utils;

#[program]
pub mod sig_verify {

    use super::*;

    pub fn signature_verify(
        ctx: Context<SignatureVerify>,
        args: SignatureVerifyArgs,
    ) -> Result<()> {
        let verify_ed25519_instruction =
            construct_ed25519_instruction(args.pubkey, args.signature, args.msg);

        invoke(
            &verify_ed25519_instruction,
            &vec![ctx.accounts.ed25519_program.to_account_info()],
        )?;

        msg!("Verify across!");

        Ok(())
    }
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
