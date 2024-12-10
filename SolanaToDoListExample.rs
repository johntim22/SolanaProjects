use anchor_lang::prelude::*;

declare_id!("CuHvrtQSkDj3VsndB59x4MRyJ5Pr4hdNR7QG18rgTNSB");

#[program]
pub mod todo_list {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let todo_list = &mut ctx.accounts.todo_list;
        todo_list.tasks = Vec::new();
        Ok(())
    }

    pub fn add_task(ctx: Context<ModifyTask>, task: String) -> Result<()> {
        let todo_list = &mut ctx.accounts.todo_list;
        todo_list.tasks.push(task);
        Ok(())
    }

    pub fn remove_task(ctx: Context<ModifyTask>, index: u32) -> Result<()> {  // Changed usize to u32
        let todo_list = &mut ctx.accounts.todo_list;
        if index < todo_list.tasks.len() as u32 {
            todo_list.tasks.remove(index as usize);  // Convert u32 to usize here
        }
        Ok(())
    }

    pub fn get_tasks(ctx: Context<ModifyTask>) -> Result<Vec<String>> {
        let todo_list = &ctx.accounts.todo_list;
        Ok(todo_list.tasks.clone())
    }
}

#[account]
pub struct TodoList {
    pub tasks: Vec<String>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 1024)]
    pub todo_list: Account<'info, TodoList>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyTask<'info> {
    #[account(mut)]
    pub todo_list: Account<'info, TodoList>,
}
