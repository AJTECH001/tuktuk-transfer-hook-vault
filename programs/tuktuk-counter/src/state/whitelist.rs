use anchor_lang::prelude::*;

#[account]
pub struct Whitelist {
    pub vault: Pubkey,
    pub user: Pubkey,
    pub is_whitelisted: bool,
    pub bump: u8,
}

impl Whitelist {
    pub const LEN: usize = 8 + 32 + 32 + 1 + 1;
}
