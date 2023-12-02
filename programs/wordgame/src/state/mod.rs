use anchor_lang::prelude::*;

pub struct Game {
    pub name: String,
    pub min_deposit: u64,
    pub owner: Pubkey,
    pub current_word: String,
    pub escrow_account: Pubkey, // Game Escrow Account
    // Add other game state variables as needed
}

pub struct Player {
    pub game_account: Pubkey,
    pub player_account: Pubkey,
    pub deposited_amount: u64,
    // Add other player state variables as needed
}
