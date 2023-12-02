use anchor_lang::prelude::*;
use crate::{Game, Player};

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,
    #[account(init, payer = player, space = 8 + 128)] // Adjust space as needed for other player state variables
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub game_account: Account<'info, Game>,
}

pub fn handle_join_game(ctx: Context<JoinGame>) -> Result<()> {
    let player = &mut ctx.accounts.player;
    player.game_account = *ctx.accounts.game_account.to_account_info().key;
    player.player_account = *ctx.accounts.player_account.to_account_info().key;
    player.deposited_amount = 0; // Initialize deposited_amount to 0

    Ok(())
}
