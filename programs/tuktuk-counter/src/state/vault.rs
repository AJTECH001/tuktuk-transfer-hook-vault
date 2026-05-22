use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub reward_interval: i64,
    pub last_reward_time: i64,
    pub bump: u8,
    pub whitelisted_users: Vec<Pubkey>,
}

impl Vault {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 1 + 4 + (32 * 100); // 100 users max
}
