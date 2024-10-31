use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct IbuidlProfile {
    pub tweet_count: u32,

    #[max_len(20)]
    pub display_name: String,
}

impl IbuidlProfile {
    pub const SEED_PREFIX: &'static str = "profile";
}
