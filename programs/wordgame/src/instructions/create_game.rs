use anchor_lang::prelude::*;
use crate::Game;

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(init, payer = owner, space = 8 + 128)] // Adjust space as needed for other game state variables
    pub game: Account<'info, Game>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_game(ctx: Context<CreateGame>, name: String, min_deposit: u64) -> Result<()> {
    let game = &mut ctx.accounts.game;
    game.name = name;
    game.min_deposit = min_deposit;
    game.owner = *ctx.accounts.owner.key;
    game.escrow_account = Pubkey::new_unique(); // Create a unique Pubkey for the Game Escrow account

    Ok(())
}
