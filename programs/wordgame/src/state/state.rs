use anchor_lang::prelude::*;

#[account]
pub struct GameData {
    pub name: String,
    pub min_deposit: u8,
    pub owner: Pubkey,
    pub total_games_won: u32,
    pub total_winnings: u32,
    pub total_players: u32,
    pub game_pot: u32,
    pub bump: u8,
}

impl GameData {
    pub const MAX_SIZE: usize = 1 + (4 + 15) + 32 + 4 + 4 + 4 + 4 + 1;
}

#[account]
pub struct Player {
    pub game_account: Pubkey,
    pub deposited_amount: u64,
    pub games_won: u16,
}

impl Player {
    pub const MAX_SIZE: usize = 32 + 8 + 2;
}

#[account]
pub struct WordVault {
    pub secret_word: String,
}

impl WordVault {
    pub const MAX_SIZE: usize = 4 + 26;
}
