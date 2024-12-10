use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("4JNSeApGwFsQjjg4DMMrvw5Vb1bBKNucWUdZhZ9ZRKXc");

#[program]
pub mod sol_transfer {
    use super::*;

    pub fn transfer_sol(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        let sender = &ctx.accounts.sender;
        let receiver = &ctx.accounts.receiver;

        // Use the system program to perform the transfer
        let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
            sender.key,
            receiver.key,
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                sender.to_account_info(),
                receiver.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    /// CHECK: We're manually transferring SOL; no checks are needed
    pub receiver: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
