pub mod instructions;
pub mod state;
mod errors;

pub use instructions::*;
pub use state::*;

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod wordgame {
    use super::*;

    /// Creates the Game Account
    pub fn initialize_create_game(ctx: Context<CreateGame>, name: String, min_deposit: u8) -> Result<()> {
        handle_create_game(ctx, name, min_deposit);
        Ok(())
    }

    /// Creates the Player Account
    pub fn initialize_join_game(ctx: Context<JoinGame>) -> Result<()> {
        handle_join_game(ctx);
        Ok(())
    }

    /// Transfers the Initial Deposit Amount
    pub fn initialize_play_game(ctx: Context<PlayGame>, deposited_amount: u64) -> Result<()> {
        handle_play_game(ctx, deposited_amount);
        Ok(())
    }

    /// Allows a Player to Claim Victory, Clears/Updates Game States
    pub fn claim_victory(ctx: Context<ClaimVictory>, secret_word: String) -> Result<()> {
        handle_claim_victory(ctx, secret_word);
        Ok(())
    }
}
