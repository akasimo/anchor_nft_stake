use anchor_lang::prelude::*;

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct InitializeAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = UserAccount::INIT_SPACE,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    system_program: Program<'info, System>,
}

impl<'info> InitializeAccount<'info> {
    pub fn init_user(&mut self, bumps: &InitializeAccountBumps) -> Result<()> {
        self.user_account.set_inner( UserAccount {
            points : 0, 
            amount_staked : 0,
            bump : bumps.user_account
        });
        Ok(())
    }
}