use anchor_lang::prelude::*;
use crate::state::{Vault, Whitelist};

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct WhitelistUser<'info> {
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
        init_if_needed,
        payer = admin,
        space = Whitelist::LEN,
        seeds = [b"whitelist", vault.key().as_ref(), user.as_ref()],
        bump,
    )]
    pub whitelist: Account<'info, Whitelist>,

    pub system_program: Program<'info, System>,
}

impl<'info> WhitelistUser<'info> {
    pub fn whitelist_user(&mut self, user: Pubkey, bump: u8) -> Result<()> {
        self.whitelist.set_inner(Whitelist {
            vault: self.vault.key(),
            user,
            is_whitelisted: true,
            bump,
        });

        if !self.vault.whitelisted_users.contains(&user) {
            self.vault.whitelisted_users.push(user);
        }
        Ok(())
    }
}
