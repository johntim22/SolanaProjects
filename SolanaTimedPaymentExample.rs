use anchor_lang::prelude::*;
use solana_program::clock::Clock;
use solana_program::sysvar::Sysvar;

declare_id!("EZjW8c3EGbPwon8N5sYaqUbG6Hn8W6WHzzP86LgqDWcA");

#[program]
pub mod installment_transfer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, recipient: Pubkey) -> Result<()> {
        let transfer_data = &mut ctx.accounts.transfer_data;
        transfer_data.recipient = recipient;
        transfer_data.first_installment_time = None;
        Ok(())
    }

    pub fn first_installment(ctx: Context<Installment>, amount: u64) -> Result<()> {
        let transfer_data = &mut ctx.accounts.transfer_data;
        let clock = Clock::get()?;
        
        // Ensure first installment hasn't already been sent
        require!(
            transfer_data.first_installment_time.is_none(),
            CustomError::FirstInstallmentAlreadySent
        );

        // Perform the first transfer
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.sender.key(),
            &transfer_data.recipient,
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.sender.to_account_info(),
                ctx.accounts.recipient.to_account_info(),
            ],
        )?;

        // Record the time of the first installment
        transfer_data.first_installment_time = Some(clock.unix_timestamp);
        Ok(())
    }

    pub fn second_installment(ctx: Context<Installment>, amount: u64) -> Result<()> {
        let transfer_data = &mut ctx.accounts.transfer_data;
        let clock = Clock::get()?;

        // Ensure the first installment has been sent
        let first_time = transfer_data
            .first_installment_time
            .ok_or(CustomError::FirstInstallmentNotSent)?;

        // Ensure at least 10 minutes have passed
        require!(
            clock.unix_timestamp >= first_time + 10 * 60,
            CustomError::TooEarlyForSecondInstallment
        );

        // Perform the second transfer
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.sender.key(),
            &transfer_data.recipient,
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.sender.to_account_info(),
                ctx.accounts.recipient.to_account_info(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 16 + 32 + 16)]
    pub transfer_data: Account<'info, TransferData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Installment<'info> {
    #[account(mut)]
    pub transfer_data: Account<'info, TransferData>,
    #[account(mut)]
    pub sender: Signer<'info>,
    /// CHECK: This is safe because we don't perform any operation on the recipient
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TransferData {
    pub recipient: Pubkey,
    pub first_installment_time: Option<i64>,
}

#[error_code]
pub enum CustomError {
    #[msg("First installment already sent.")]
    FirstInstallmentAlreadySent,
    #[msg("First installment not sent yet.")]
    FirstInstallmentNotSent,
    #[msg("Too early for second installment.")]
    TooEarlyForSecondInstallment,
}
