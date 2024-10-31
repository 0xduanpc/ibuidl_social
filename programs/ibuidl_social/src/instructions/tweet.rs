use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::state::{like::IbuidlLike, profile::IbuidlProfile, tweet::IbuidlTweet};

pub fn create_tweet(ctx: Context<CreateTweet>, body: String) -> Result<()> {
    let profile = &mut ctx.accounts.profile;
    profile.tweet_count += 1;

    let tweet = IbuidlTweet::new(body, ctx.accounts.authority.key());
    ctx.accounts.tweet.set_inner(tweet.clone());

    Ok(())
}

#[derive(Accounts)]
pub struct CreateTweet<'info> {
    #[account(
        init,
        payer = authority,
        // 这里虽然不+8也能执行，是因为保存的String类型没有填满，填满了就会报错
        space = 8 + IbuidlTweet::INIT_SPACE,
        seeds = [
            IbuidlTweet::SEED_PREFIX.as_bytes(),
            profile.key().as_ref(),
            (profile.tweet_count + 1).to_string().as_ref()
        ],
        bump
    )]
    pub tweet: Account<'info, IbuidlTweet>,

    #[account(
        mut,
        seeds = [
            IbuidlProfile::SEED_PREFIX.as_bytes(),
            authority.key().as_ref()
        ],
        bump
    )]
    pub profile: Account<'info, IbuidlProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_like(ctx: Context<CreateLike>) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    tweet.like_count += 1;

    let like_rel = IbuidlLike::new(ctx.accounts.profile.key(), ctx.accounts.tweet.key());
    ctx.accounts.like.set_inner(like_rel.clone());

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.author_token_account.to_account_info(),
                authority: ctx.accounts.mint_account.to_account_info(),
            },
            &[&[b"mint_v9", &[ctx.bumps.mint_account]]],
        ),
        1000000,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateLike<'info> {
    #[account(
        mut,
        seeds = [
            b"mint_v9",
        ],
        bump,
    )]
    pub mint_account: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint_account,
        associated_token::authority = author_wallet,
    )]
    pub author_token_account: Account<'info, TokenAccount>,

    // 这里先不校验了
    /// CHECK: This is author
    pub author_wallet: AccountInfo<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + IbuidlLike::INIT_SPACE,
        seeds = [
            IbuidlLike::SEED_PREFIX.as_bytes().as_ref(),
            profile.key().as_ref(),
            tweet.key().as_ref()
        ],
        bump
    )]
    pub like: Account<'info, IbuidlLike>,

    #[account(mut)]
    pub tweet: Account<'info, IbuidlTweet>,

    #[account(
        mut,
        seeds = [
            IbuidlProfile::SEED_PREFIX.as_bytes(),
            authority.key().as_ref()
        ],
        bump
    )]
    pub profile: Account<'info, IbuidlProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
