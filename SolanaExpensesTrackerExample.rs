use anchor_lang::prelude::*;

declare_id!("HC6ihM4nEXr17GRQhWNe8v9DsietUVopoSyFuaVhsjjn"); // Replace with the program ID after deploying.

#[program]
pub mod expense_tracker {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let expense_account = &mut ctx.accounts.expense_account;
        expense_account.total_expense = 0;
        Ok(())
    }

    pub fn add_expense(ctx: Context<AddExpense>, amount: u64, description: String, date: i64) -> Result<()> {
    let expense_account = &mut ctx.accounts.expense_account;
    expense_account.total_expense += amount;
    expense_account.entries.push(ExpenseEntry { amount, description, date });
    Ok(())
}


    pub fn remove_all_expenses(ctx: Context<ModifyExpenses>) -> Result<()> {
    let expense_account = &mut ctx.accounts.expense_account;
    expense_account.entries.clear();
    expense_account.total_expense = 0;
    Ok(())
}

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 1024)]
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

#[derive(Accounts)]
pub struct ModifyExpenses<'info> {
    #[account(mut)]
    pub expense_account: Account<'info, ExpenseAccount>,
}


#[account]
pub struct ExpenseAccount {
    pub total_expense: u64,
    pub entries: Vec<ExpenseEntry>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ExpenseEntry {
    pub amount: u64,
    pub description: String,
    pub date: i64,
}
