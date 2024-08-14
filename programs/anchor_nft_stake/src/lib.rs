use anchor_lang::prelude::*;

mod state;
mod instructions;

pub mod errors;

// pub use errors::*;
pub use instructions::*;
pub use state::*;

declare_id!("DnV3BqazunEc7o2voPPVLJro79xhBNLZiubFknyiVKFf");

#[program]
pub mod anchor_nft_stake {
    use super::*;

    pub fn initialize(ctx: Context<InitializeConfig>) -> Result<()> {
        // ctx.accounts.init_config(&ctx.bumps)?;
        Ok(())
    }

    pub fn initialize_account(ctx: Context<InitializeAccount>) -> Result<()> {
        ctx.accounts.init_user(&ctx.bumps)?;
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct Initialize {}
