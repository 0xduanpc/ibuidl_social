use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct IbuidlTweet {
    pub like_count: u64,
    pub author: Pubkey,
    #[max_len(50)]
    pub body: String,
}

impl IbuidlTweet {
    pub const SEED_PREFIX: &'static str = "tweet";

    pub fn new(body: String, author: Pubkey) -> Self {
        Self {
            body,
            like_count: 0,
            author: author,
        }
    }
}
