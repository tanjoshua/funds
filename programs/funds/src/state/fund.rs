use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Fund {
    pub id: u64,
    pub owner: Pubkey,
    #[max_len(50)]
    pub name: String,

    // token mint of the accepted fund token
    pub fund_input_token_mint: Pubkey,

    // mint of the token representing fund ownership
    pub fund_ownership_token_mint: Pubkey,

    // used to calculate the PDA of the account
    pub bump: u8,
}
