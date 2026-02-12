use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::{errors::AmmError, state::Config};



#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Init<'info> {
    #[account(mut)]
    pub init_user: Signer<'info>,
    pub mint_token_x: Account<'info, Mint>,
    pub mint_token_y: Account<'info, Mint>,
    #[account(
        init,
        payer = init_user,
        seeds = [b"lp", config.key.as_ref()],
        bump,
        mint::decimals = mint_token_x.decimals,
        mint::authority = config,
    )]
    pub mint_lp_token: Account<'info, Mint>,
    #[account(
        init,
        payer = init_user,
        associated_token::mint = mint_token_x,
        associated_token::authority = config,
    )]
    pub vault_token_x: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = init_user,
        associated_token::mint = mint_token_y,
        associated_token::authority = config,
    )]
    pub vault_token_y: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = init_user,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = 8 + Config::INIT_SPACE,
    )]
    pub config: Account<'info, Config>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Init<'info> {
    pub fn init(&mut self, seed: u64, fee: u16, authority: Option<Pubkey>, bumps: InitBumps) -> Result<()> {
        // Fee is in basis points (1/100th of a percent). Max 100% = 10000 bps.
        require!(fee <= 10000, AmmError::InvalidFee);

        self.config.set_inner(Config {
            seed,
            authority,
            mint_x: self.mint_token_x.key(),
            mint_y: self.mint_token_y.key(),
            fee,
            locked: false,
            config_bump: bumps.config,
            lp_bump: bumps.mint_lp_token,
        });

        Ok(())
    }
}