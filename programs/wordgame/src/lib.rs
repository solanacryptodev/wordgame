pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod wordgame {
    use super::*;

    pub fn initialize_create_game(ctx: Context<CreateGame>, name: String, min_deposit: u64) -> ProgramResult {
        handle_create_game(ctx, name, min_deposit);

        Ok(())
    }

    pub fn initialize_join_game(ctx: Context<JoinGame>) -> ProgramResult {
        handle_join_game(ctx);

        Ok(())
    }

    pub fn initialize_play_game(ctx: Context<PlayGame>, deposited_amount: u64) -> ProgramResult {
        handle_play_game(ctx, deposited_amount);

        Ok(())
    }

    pub fn initialize_random_word(ctx: Context<ChooseRandomWord>) -> ProgramResult {
        handle_random_word(ctx);

        Ok(())
    }

    pub fn reset_word(ctx: Context<ResetWord>) -> ProgramResult {
        handle_reset_word(ctx);

        Ok(())
    }
}
