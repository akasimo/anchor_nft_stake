use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::{mpl_token_metadata::instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts, ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata, MetadataAccount}, token::{approve, mint_to, revoke, Approve, Mint, MintTo, Revoke, Token, TokenAccount}};
use crate::state::{StakeAccount, StakeConfig, UserAccount};

use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub config: Account<'info, StakeConfig>,

    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,


    #[account(
        mut,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump = config.rewards_bump,
    )]
    pub rewards_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user,
    )]
    pub rewards_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.rewards_mint.to_account_info(),
            to: self.rewards_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        let amount = (self.user_account.points as u64) * 10_u64.pow(self.rewards_mint.decimals as u32);
        mint_to(cpi_ctx, amount)?;
        self.user_account.points = 0;
        Ok(())
    }
}
