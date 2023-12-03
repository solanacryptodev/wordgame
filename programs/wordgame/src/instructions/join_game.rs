use anchor_lang::prelude::*;
use crate::state::state::{GameData, Player};

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(
        init_if_needed,
        payer = player,
        space = 8 + 128
    )] // Adjust space as needed for other player state variables
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub game_account: Account<'info, GameData>,
}

pub fn handle_join_game(ctx: Context<JoinGame>) -> Result<()> {
    let player = &mut ctx.accounts.player;
    player.game_account = *ctx.accounts.game_account.to_account_info().key;
    msg!("Joined Game: {}", player.game_account);

    player.player_account = *ctx.accounts.player_account.to_account_info().key;
    msg!("Your Player Account: {}", player.player_account);

    player.deposited_amount = 0;
    msg!("Initial Deposit Amount: {}", player.deposited_amount);

    Ok(())
}
