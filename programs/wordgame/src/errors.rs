use anchor_lang::error_code;

#[error_code]
pub enum WordGameErrors {
    /// Secret Word Mismatch
    #[msg("The provided secret word does not match the stored secret word.")]
    SecretWordMismatch,
}
