use anchor_lang::prelude::*;
use crate::state::{Vault, Whitelist};

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct RemoveWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault"],
        bump = vault.bump,
        has_one = admin,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"whitelist", vault.key().as_ref(), user.as_ref()],
        bump = whitelist.bump,
    )]
    pub whitelist: Account<'info, Whitelist>,
}

impl<'info> RemoveWhitelist<'info> {
    pub fn remove_whitelist(&mut self) -> Result<()> {
        self.whitelist.is_whitelisted = false;
        self.vault.whitelisted_users.retain(|&x| x != self.whitelist.user);
        Ok(())
    }
}
