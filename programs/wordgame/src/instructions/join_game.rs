use anchor_lang::prelude::*;
use crate::state::state::{GameData, Player};

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(
        init_if_needed,
        payer = owner,
        space = Player::MAX_SIZE
    )] // Adjust space as needed for other player state variables
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub game_account: Account<'info, GameData>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_join_game(ctx: Context<JoinGame>) -> Result<()> {
    let player = &mut ctx.accounts.player;
    let game = &mut ctx.accounts.game_account;

    game.total_players += 1;

    /// Game account that the player is joining
    player.game_account = *ctx.accounts.game_account.to_account_info().key;
    msg!("Joined Game: {}", player.game_account);

    player.games_won = *ctx.accounts.player.games_won;
    msg!("Your Player Account: {}", player.games_won);

    player.deposited_amount = 0;
    msg!("Initial Deposit Amount: {}", player.deposited_amount);

    Ok(())
}
