use anchor_lang::prelude::*;

declare_id!("BbKjaqtsVBcReSQaUC9qSFzPLCLELtKDTUEhQ1swFiQ1");

#[program]
pub mod expenses {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let expense_account = &mut ctx.accounts.expense_account;
        expense_account.expenses = Vec::new();
        Ok(())
    }

    pub fn add_expense(ctx: Context<AddExpense>, amount: u64, description: String, merchant: String, date: String) -> Result<()> {
        let expense_account = &mut ctx.accounts.expense_account;

        require!(description.len() <= 100, CustomError::DescriptionTooLong);
        require!(date.len() <= 10, CustomError::DateTooLong);

        let expense = Expense {
            amount,
            description,
            merchant,
            date,
        };

        expense_account.expenses.push(expense);
        Ok(())
    }

    pub fn delete_expense(ctx: Context<DeleteExpense>, index: u64) -> Result<()> {
    let expense_account = &mut ctx.accounts.expense_account;

    require!(
        (index as usize) < expense_account.expenses.len(),
        CustomError::InvalidIndex
    );

    expense_account.expenses.remove(index as usize);
    Ok(())
    }

    pub fn reset_expenses(ctx: Context<ResetExpenses>) -> Result<()> {
    let expense_account = &mut ctx.accounts.expense_account;
    expense_account.expenses.clear();
    Ok(())
    }


}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 1000)] // Adjust space as needed
    pub expense_account: Account<'info, ExpenseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddExpense<'info> {
    #[account(mut)]
    pub expense_account: Account<'info, ExpenseAccount>,
}

#[account]
pub struct ExpenseAccount {
    pub expenses: Vec<Expense>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Expense {
    pub amount: u64,
    pub description: String,
    pub merchant: String,
    pub date: String,
}

#[derive(Accounts)]
pub struct DeleteExpense<'info> {
    #[account(mut)]
    pub expense_account: Account<'info, ExpenseAccount>,
}

#[derive(Accounts)]
pub struct ResetExpenses<'info> {
    #[account(mut)]
    pub expense_account: Account<'info, ExpenseAccount>,
}



#[error_code]
pub enum CustomError {
    #[msg("Description exceeds 100 characters")]
    DescriptionTooLong,
    #[msg("Date exceeds 10 characters")]
    DateTooLong,
    #[msg("Invalid expense index")]
    InvalidIndex,
}

