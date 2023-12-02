use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ResetWord<'info> {
    // Define accounts needed for the reset_word instruction
}

pub fn handle_reset_word(ctx: Context<ResetWord>) -> Result<()> {
    // Handle the reset_word instruction
    Ok(())
}
