use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, TransferChecked, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::state::{Vault, Whitelist};
use crate::errors::CounterError;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"vault"],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        seeds = [b"whitelist", vault.key().as_ref(), user.key().as_ref()],
        bump = whitelist.bump,
        constraint = whitelist.is_whitelisted @ CounterError::NotWhitelisted,
    )]
    pub whitelist: Account<'info, Whitelist>,

    #[account(
        mut,
        address = vault.mint,
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault,
        associated_token::token_program = token_program,
    )]
    pub vault_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        token_2022::transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.user_token_account.to_account_info(),
                    mint: self.mint.to_account_info(),
                    to: self.vault_token_account.to_account_info(),
                    authority: self.user.to_account_info(),
                },
            ),
            amount,
            self.mint.decimals,
        )?;

        Ok(())
    }
}
