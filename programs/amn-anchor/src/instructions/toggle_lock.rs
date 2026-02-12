use anchor_lang::prelude::*;

use crate::{errors::AmmError, state::Config};

#[derive(Accounts)]
pub struct ToggleLock<'info> {
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,
}

impl<'info> ToggleLock<'info> {
    pub fn toggle_lock(&mut self) -> Result<()> {
        let authority = self.config.authority.ok_or(AmmError::NoAuthoritySet)?;
        require!(self.authority.key() == authority, AmmError::Unauthorized);
        self.config.locked = !self.config.locked;
        Ok(())
    }
}
