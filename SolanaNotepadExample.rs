use anchor_lang::prelude::*;

declare_id!("5m7bNoCZLwLLH8dEUDdiA9NNa7iUHjqBGvebLYCCb8Wq");

#[program]
pub mod solana_anchor_notepad {
    use super::*;

    pub fn create_note(ctx: Context<CreateNote>, title: String, content: String, date: i64, time: i64) -> Result<()> {
        let note = &mut ctx.accounts.note;
        note.title = title;
        note.content = content;
        note.date = date;
        note.time = time;
        msg!("Note created: Title: {}, Content: {}, Date:{}, Time: {}",note.title,note.content,note.date,note.time);
        Ok(())
    }

    pub fn edit_note(ctx: Context<EditNote>, title: String, content: String) -> Result<()> {
        let note = &mut ctx.accounts.note;
        note.title = title;
        note.content = content;
        msg!("Note edited: Title: {}, Content: {}", note.title, note.content);
        Ok(())
    }

    pub fn delete_note(_ctx: Context<DeleteNote>) -> Result<()> {
        let note = &_ctx.accounts.note;
        msg!("Note deleted: Title: {}", note.title);
        Ok(())
    }

    pub fn reset_notepad(ctx: Context<ResetNotepad>) -> Result<()> {
        let notes = &mut ctx.accounts.notepad;
        notes.notes.clear();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateNote<'info> {
    #[account(init, payer = user, space = 8 + 32 + 200 + 8 + 8)]
    pub note: Account<'info, Note>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EditNote<'info> {
    #[account(mut)]
    pub note: Account<'info, Note>,
}

#[derive(Accounts)]
pub struct DeleteNote<'info> {
    #[account(mut, close = user)]
    pub note: Account<'info, Note>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResetNotepad<'info> {
    #[account(mut)]
    pub notepad: Account<'info, Notepad>,
}

#[account]
pub struct Note {
    pub title: String,
    pub content: String,
    pub date: i64,
    pub time: i64,
}

#[account]
pub struct Notepad {
    pub notes: Vec<Pubkey>, // Store public keys of notes
}
