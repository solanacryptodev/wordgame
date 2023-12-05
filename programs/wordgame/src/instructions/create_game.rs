use anchor_lang::prelude::*;
use random_word::Lang;
use crate::state::state::{GameData, WordVault};

#[derive(Accounts)]
pub struct CreateGame<'info> {
    #[account(
        init,
        payer = owner,
        space = GameData::MAX_SIZE,
        seeds = [b"game", user.key().as_ref()],
        bump
    )]
    pub game: Account<'info, GameData>,
    #[account(
        init_if_needed,
        seeds = [b"wordVault"],
        bump,
        payer = owner,
        space = WordVault::MAX_SIZE
    )]
    pub word_vault: Account<'info, WordVault>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_game(ctx: Context<CreateGame>, name: String, min_deposit: u8) -> Result<()> {
    let game = &mut ctx.accounts.game;
    let word_vault = &mut ctx.accounts.word_vault;

    let word = random_word::gen(Lang::En);

    word_vault.secret_word = String::from(word);

    game.name = name;
    msg!("Game name: {}", game.name);

    game.min_deposit = min_deposit;
    msg!("Minimum deposit: {}", game.min_deposit);

    game.owner = *ctx.accounts.owner.key;
    msg!("Game owner: {}", game.owner);

    game.game_pot = 0;
    msg!("Game escrow account: {}", game.game_pot);

    game.total_games_won = 0;
    msg!("Total Games Won: {}", game.total_games_won);

    game.total_winnings = 0;
    msg!("Total Winnings: {}", game.total_winnings);

    Ok(())
}
