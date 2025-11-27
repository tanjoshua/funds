use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::Fund;


#[derive(Accounts)]
#[instruction(fund_name: String, id: u64)]
pub struct CreateFund<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(init, 
        payer = owner,
        mint::decimals = 6,
        mint::authority = fund, // Fund PDA controls the mint
        mint::freeze_authority = fund,
     )]
    pub ownership_token_mint: Account<'info, Mint>,

    pub input_token_mint: Account<'info, Mint>, // Accepted token (e.g. USDC)

    #[account(init,
        payer = owner,
        space = 8 + Fund::INIT_SPACE,
        seeds = [b"fund", id.to_le_bytes().as_ref()],
        bump,
    )]
    pub fund: Account<'info, Fund>,

    #[account(init,
        payer = owner,
        associated_token::mint = input_token_mint,
        associated_token::authority = fund,
    )]
    pub fund_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateFund>, fund_name: String, id: u64) -> Result<()> {
    let fund = &mut ctx.accounts.fund;
    
    fund.id = id;
    fund.owner = ctx.accounts.owner.key();
    fund.name = fund_name;
    fund.fund_input_token_mint = ctx.accounts.input_token_mint.key();
    fund.fund_ownership_token_mint = ctx.accounts.ownership_token_mint.key();
    fund.bump = ctx.bumps.fund;

    msg!("Created fund: {} with ID: {}", fund.name, fund.id);
    Ok(())
}
