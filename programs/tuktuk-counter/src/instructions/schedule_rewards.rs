use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::{prelude::*, InstructionData};
use tuktuk_program::{
    compile_transaction,
    tuktuk::{
        cpi::{accounts::QueueTaskV0, queue_task_v0},
        program::Tuktuk,
        types::TriggerV0,
    },
    types::QueueTaskArgsV0,
    TransactionSourceV0,
};
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::Mint;
use crate::state::Vault;

#[derive(Accounts)]
pub struct ScheduleRewards<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"vault"],
        bump = vault.bump,
        has_one = admin,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        address = vault.mint,
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    /// CHECK: The task queue
    pub task_queue: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: The task queue authority
    pub task_queue_authority: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: The task
    pub task: UncheckedAccount<'info>,
    
    /// CHECK: Via seeds
    #[account(
        mut,
        seeds = [b"queue_authority"],
        bump
    )]
    pub queue_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub tuktuk_program: Program<'info, Tuktuk>,
}

impl<'info> ScheduleRewards<'info> {
    pub fn schedule_rewards(&mut self, task_id: u16, cron: String, bumps: ScheduleRewardsBumps, remaining_accounts: &[AccountInfo<'info>]) -> Result<()> {
        // Prepare the instruction to be scheduled
        let mut accounts = vec![
            AccountMeta::new(self.vault.key(), false),
            AccountMeta::new(self.mint.key(), false),
            AccountMeta::new_readonly(self.token_program.key(), false),
        ];
        
        // Add user ATAs as remaining accounts
        for account in remaining_accounts {
            accounts.push(AccountMeta::new(account.key(), false));
        }

        let (compiled_tx, _) = compile_transaction(
            vec![Instruction {
                program_id: crate::ID,
                accounts,
                data: vec![127, 240, 241, 115, 12, 114, 255, 102], // global:cron_reward_distribution
            }],
            vec![],
        )
        .unwrap();

        queue_task_v0(
            CpiContext::new_with_signer(
                self.tuktuk_program.to_account_info(),
                QueueTaskV0 {
                    payer: self.admin.to_account_info(),
                    queue_authority: self.queue_authority.to_account_info(),
                    task_queue: self.task_queue.to_account_info(),
                    task_queue_authority: self.task_queue_authority.to_account_info(),
                    task: self.task.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                },
                &[&["queue_authority".as_bytes(), &[bumps.queue_authority]]],
            ),
            QueueTaskArgsV0 {
                trigger: TriggerV0::Now,
                transaction: TransactionSourceV0::CompiledV0(compiled_tx),
                crank_reward: Some(1000000), // 1 SOL reward for the cranker
                free_tasks: 0,
                id: task_id,
                description: "Vault Reward Distribution".to_string(),
            },
        )?;

        Ok(())
    }
}
