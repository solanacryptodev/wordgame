use anchor_lang::prelude::*;
use random_word::Lang;
use crate::errors::WordGameErrors;
use crate::state::state::{GameData, Player, WordVault};

#[derive(Accounts)]
pub struct ClaimVictory<'info> {
    #[account(mut, payer = owner)]
    pub game: Account<'info, GameData>,
    #[account(mut, payer = owner)]
    pub word_vault: Account<'info, WordVault>,
    #[account(mut)]
    pub player: Account<'info, Player>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_claim_victory(ctx: Context<ClaimVictory>, secret_word: String) -> Result<()> {
    let word = &mut ctx.accounts.word_vault;
    let game = &mut ctx.accounts.game;
    let player = &mut ctx.accounts.player;
    let new_word = random_word::gen(Lang::En);

    if secret_word == *word.secret_word {
        game.total_games_won += 1;
        player.games_won += 1;
        game.total_winnings += game.game_pot;
        game.game_pot = 0;
        word.secret_word = String::from(new_word);

        Ok(())
    } else {
        Err(WordGameErrors::SecretWordMismatch.into())
    }
}
