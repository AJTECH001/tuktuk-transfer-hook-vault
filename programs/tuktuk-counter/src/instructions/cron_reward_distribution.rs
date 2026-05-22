use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, MintTo, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::state::Vault;
use crate::errors::CounterError;

#[derive(Accounts)]
pub struct CronRewardDistribution<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        address = vault.mint,
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Program<'info, Token2022>,
}

impl<'info> CronRewardDistribution<'info> {
    pub fn distribute_rewards(&mut self, remaining_accounts: &[AccountInfo<'info>]) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        require!(
            now >= self.vault.last_reward_time + self.vault.reward_interval,
            CounterError::RewardTooEarly
        );

        let vault_bump = self.vault.bump;
        let seeds = &[
            b"vault".as_ref(),
            &[vault_bump],
        ];
        let signer = &[&seeds[..]];

        // In a real production app, we would use Tuktuk to trigger this for each user 
        // or use a more efficient way to resolve token accounts.
        // For this demo, we'll iterate through whitelisted users and expect their 
        // token accounts to be passed in remaining_accounts in the same order.
        
        for (i, _user_pubkey) in self.vault.whitelisted_users.iter().enumerate() {
            if i >= remaining_accounts.len() {
                break;
            }
            
            let user_ata = &remaining_accounts[i];
            
            // Basic validation of the ATA
            // In production, we'd check owner == user_pubkey and mint == vault.mint
            
            token_2022::mint_to(
                CpiContext::new_with_signer(
                    self.token_program.to_account_info(),
                    MintTo {
                        mint: self.mint.to_account_info(),
                        to: user_ata.clone(),
                        authority: self.vault.to_account_info(),
                    },
                    signer,
                ),
                100, // Reward amount
            )?;
        }

        self.vault.last_reward_time = now;
        Ok(())
    }
}
