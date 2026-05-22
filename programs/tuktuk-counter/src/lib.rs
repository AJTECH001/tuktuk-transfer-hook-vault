use anchor_lang::prelude::*;

declare_id!("DTk2fbtzc4JkfCXdYYwQNjyigCXwZtBrSPL8sDnmUGj9");
mod state;
mod instructions;
mod errors;

pub use instructions::*;
pub use errors::*;

#[program]
pub mod tuktuk_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.increment_counter()
    }

    pub fn schedule(ctx: Context<Schedule>, task_id: u16) -> Result<()> {
        ctx.accounts.schedule(task_id, ctx.bumps)
    }

    pub fn initialize_vault(ctx: Context<InitializeVault>, reward_interval: i64) -> Result<()> {
        ctx.accounts.initialize_vault(reward_interval, ctx.bumps.vault)
    }

    pub fn whitelist_user(ctx: Context<WhitelistUser>, user: Pubkey) -> Result<()> {
        ctx.accounts.whitelist_user(user, ctx.bumps.whitelist)
    }

    pub fn remove_whitelist(ctx: Context<RemoveWhitelist>, _user: Pubkey) -> Result<()> {
        ctx.accounts.remove_whitelist()
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    pub fn initialize_extra_account_meta_list(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
        initialize_extra_account_meta_list::handler(ctx)
    }

    #[interface(spl_transfer_hook_interface::execute)]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        transfer_hook::handler(ctx, amount)
    }

    pub fn cron_reward_distribution<'info>(ctx: Context<'_, '_, '_, 'info, CronRewardDistribution<'info>>) -> Result<()> {
        ctx.accounts.distribute_rewards(ctx.remaining_accounts)
    }

    pub fn schedule_rewards<'info>(ctx: Context<'_, '_, '_, 'info, ScheduleRewards<'info>>, task_id: u16, cron: String) -> Result<()> {
        ctx.accounts.schedule_rewards(task_id, cron, ctx.bumps, ctx.remaining_accounts)
    }
}
