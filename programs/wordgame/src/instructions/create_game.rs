use anchor_lang::prelude::*;
use crate::state::state::{GameData, WordVault};

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 128,
        seeds = [b"game", user.key().as_ref()],
        bump
    )]
    pub game: Account<'info, GameData>,
    #[account(
        init_if_needed,
        seeds = [b"wordVault"],
        bump,
        payer = owner,
        space = 8
    )]
    pub word_vault: Account<'info, WordVault>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_game(ctx: Context<CreateGame>, name: String, min_deposit: u64) -> Result<()> {
    let game = &mut ctx.accounts.game;
    game.name = name;
    msg!("Game name: {}", game.name);

    game.min_deposit = min_deposit;
    msg!("Minimum deposit: {}", game.min_deposit);

    game.owner = *ctx.accounts.owner.key;
    msg!("Game owner: {}", game.owner);

    game.escrow_account = Pubkey::new_unique(); // Create a unique Pubkey for the Game Escrow account
    msg!("Game escrow account: {}", game.escrow_account);

    Ok(())
}
