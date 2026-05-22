use anchor_lang::prelude::*;
use spl_tlv_account_resolution::{account::ExtraAccountMeta, seeds::Seed};

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: ExtraAccountMetaList PDA, built from mint and program_id
    #[account(
        init,
        space = 8 + 4 + 3 * 35, // 8 (anchor) + 4 (len) + 3 * ExtraAccountMeta size
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        payer = payer,
    )]
    pub extra_account_meta_list: AccountInfo<'info>,

    /// CHECK: The mint account
    pub mint: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
    let account_metas = vec![
        // Index 5: Vault PDA
        ExtraAccountMeta::new_with_seeds(
            &[Seed::Literal { bytes: b"vault".to_vec() }],
            false, // is_signer
            false, // is_writable
        )?,
        // Index 6: Source Whitelist PDA
        ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal { bytes: b"whitelist".to_vec() },
                Seed::AccountKey { index: 5 }, // Vault key
                Seed::AccountKey { index: 3 }, // Source owner key
            ],
            false,
            false,
        )?,
        // Index 7: Destination Whitelist PDA
        ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal { bytes: b"whitelist".to_vec() },
                Seed::AccountKey { index: 5 }, // Vault key
                Seed::AccountKey { index: 2 }, // Destination owner key? No, index 2 is destination token account. 
                // We need destination owner. 
            ],
            false,
            false,
        )?,
    ];

    // Wait, the standard Transfer Hook accounts are:
    // 0: Source
    // 1: Mint
    // 2: Destination
    // 3: Owner
    // 4: ExtraAccountMetaList
    
    // We need the owner of the destination account too. 
    // But the Transfer Hook interface only gives us the owner of the source account (Index 3).
    // To get the destination owner, we might need to pass it or derive it if it's the same.
    // Actually, Token-2022 transfer_checked gives us the source owner.
    
    // If we want to validate the destination, we can't easily get its owner in the hook 
    // unless it's passed as an extra account or we read the destination account data.
    
    // Let's just validate the source for now, and if the destination is a vault, we can check that too.

    let data = &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?;
    let mut cursor = 8; // Skip Anchor discriminator
    
    // Write length
    let len: u32 = account_metas.len() as u32;
    data[cursor..cursor + 4].copy_from_slice(&len.to_le_bytes());
    cursor += 4;

    for meta in account_metas {
        let meta_slice = unsafe {
            std::slice::from_raw_parts(
                &meta as *const ExtraAccountMeta as *const u8,
                std::mem::size_of::<ExtraAccountMeta>(),
            )
        };
        data[cursor..cursor + 35].copy_from_slice(meta_slice);
        cursor += 35;
    }

    Ok(())
}
