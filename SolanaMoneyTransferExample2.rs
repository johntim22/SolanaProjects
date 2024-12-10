use anchor_lang::prelude::*;

declare_id!("A9vyHRFAk7UvLqz3wHmU6dhq1GgAjwQk2QctQfgp1PRC");

#[program]
pub mod sol_transfer {
    use super::*;
    
    pub fn transfer_sol(ctx: Context<TransferSol>, amount: u64) -> Result<()> {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.owner.key(),
            &ctx.accounts.recipient.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.recipient.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferSol<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: This is a simple transfer, so we're not validating recipient
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
