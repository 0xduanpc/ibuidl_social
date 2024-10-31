use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("5U1JcXgwu8D7S2sWN15fDwfEC4fXqM2wn6gHNgba9HXi");

#[program]
pub mod ibuidl_social {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, display_name: String) -> Result<()> {
        instructions::profile::create_profile(ctx, display_name)
    }

    pub fn create_tweet(ctx: Context<CreateTweet>, body: String) -> Result<()> {
        instructions::tweet::create_tweet(ctx, body)
    }

    pub fn create_like(ctx: Context<CreateLike>) -> Result<()> {
        instructions::tweet::create_like(ctx)
    }

    pub fn create_token_mint_account(ctx: Context<CreateTokenMintAccount>) -> Result<()> {
        instructions::token::create_token_mint_account(ctx)
    }
}
