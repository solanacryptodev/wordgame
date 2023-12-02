use anchor_lang::prelude::*;
use crate::{Game, Player};

#[derive(Accounts)]
pub struct PlayGame<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(signer, authority = from)]
    pub from: AccountInfo<'info>,
    #[account(mut)]
    pub to: Account<'info, Game>,
    #[account(mut)]
    pub transfer_authority: AccountInfo<'info>,
}

pub fn handle_play_game(ctx: Context<PlayGame>, deposited_amount: u64) -> Result<()> {
    let player = &mut ctx.accounts.player;
    let game = &ctx.accounts.game;

    // Transfer funds to the Game Escrow account
    **ctx.accounts.to = game.escrow_account;
    **ctx.accounts.transfer_authority = game.owner;

    // Update player's deposited amount
    player.deposited_amount += deposited_amount;

    Ok(())
}
