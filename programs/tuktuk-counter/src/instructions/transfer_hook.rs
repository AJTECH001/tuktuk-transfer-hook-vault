use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::state::{Vault, Whitelist};
use crate::errors::CounterError;

#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(
        token::mint = mint,
        token::authority = owner,
    )]
    pub source: Box<InterfaceAccount<'info, TokenAccount>>,
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        token::mint = mint,
    )]
    pub destination: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: The source owner
    pub owner: UncheckedAccount<'info>,
    /// CHECK: ExtraAccountMetaList PDA
    #[account(
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,

    #[account(
        seeds = [b"vault"],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        seeds = [b"whitelist", vault.key().as_ref(), owner.key().as_ref()],
        bump = source_whitelist.bump,
    )]
    pub source_whitelist: Account<'info, Whitelist>,
}

pub fn handler(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
    // If the owner is the vault, we allow it (e.g. for rewards or withdrawals)
    if ctx.accounts.owner.key() == ctx.accounts.vault.key() {
        return Ok(());
    }

    // Otherwise, check if the source owner is whitelisted
    if !ctx.accounts.source_whitelist.is_whitelisted {
        return Err(CounterError::NotWhitelisted.into());
    }

    Ok(())
}
