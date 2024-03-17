use spl_token_2022::extension::confidential_transfer::instruction::initialize_mint;
use spl_token_2022::solana_zk_token_sdk::zk_token_elgamal::pod::ElGamalPubkey;
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::Result;
use anchor_lang::{context::CpiContext, Accounts};

pub fn confidential_transfer_initialize<'info>(
    ctx: CpiContext<'_, '_, '_, 'info, ConfidentialTransferInitialize<'info>>,
    authority: Option<Pubkey>,
    auto_approve_new_accounts: bool,
    auditor_elgamal_pubkey: Option<ElGamalPubkey>
) -> Result<()> {
    let ix =  initialize_mint(
        ctx.accounts.token_program_id.key,
        ctx.accounts.mint.key,
        authority,
        auto_approve_new_accounts,
        auditor_elgamal_pubkey
    )?;
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.token_program_id,
            ctx.accounts.mint,
        ],
        ctx.signer_seeds,
    )
        .map_err(Into::into)
}

#[derive(Accounts)]
pub struct ConfidentialTransferInitialize<'info> {
    pub token_program_id: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
}
