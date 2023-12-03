use anchor_lang::prelude::*;

#[account]
pub struct GameData {
    pub name: String,
    pub min_deposit: u64,
    pub owner: Pubkey,
    pub current_word: String,
    pub bump: u8,
}

#[account]
pub struct Player {
    pub game_account: Pubkey,
    pub player_account: Pubkey,
    pub deposited_amount: u64,
}

#[account]
pub struct WordVault {}
