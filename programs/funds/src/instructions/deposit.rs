use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount, Transfer},
};

use crate::Fund;

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// The user depositing tokens into the fund
    #[account(mut)]
    pub user: Signer<'info>,

    /// The fund account (PDA) - we need it to sign for minting
    /// Anchor loads the account first, then validates that the seeds match
    #[account(
        seeds = [b"fund", fund.id.to_le_bytes().as_ref()],
        bump = fund.bump,
    )]
    pub fund: Account<'info, Fund>,

    /// The token the user is depositing (e.g., USDC)
    /// We verify it matches what the fund accepts
    #[account(
        address = fund.fund_input_token_mint
    )]
    pub input_token_mint: Account<'info, Mint>,

    /// The ownership token mint - fund PDA is the authority
    /// We'll mint new tokens to the user from this
    #[account(
        mut, // mut because supply changes when we mint
        address = fund.fund_ownership_token_mint
    )]
    pub ownership_token_mint: Account<'info, Mint>,

    /// User's token account for the INPUT token (where tokens come FROM)
    #[account(
        mut,
        associated_token::mint = input_token_mint,
        associated_token::authority = user,
    )]
    pub user_input_token_account: Account<'info, TokenAccount>,

    /// User's token account for OWNERSHIP tokens (where tokens go TO)
    /// init_if_needed creates it if user doesn't have one yet
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = ownership_token_mint,
        associated_token::authority = user,
    )]
    pub user_ownership_token_account: Account<'info, TokenAccount>,

    /// The fund's vault that holds all deposited tokens
    #[account(
        mut,
        associated_token::mint = input_token_mint,
        associated_token::authority = fund,
    )]
    pub fund_vault: Account<'info, TokenAccount>,

    // Required programs for token operations
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // Transfer input tokens from user to fund vault
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_input_token_account.to_account_info(),
            to: ctx.accounts.fund_vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, amount)?;

    // Build signer seeds for minting
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"fund",
        &ctx.accounts.fund.id.to_le_bytes(),
        &[ctx.accounts.fund.bump],
    ]];

    // Create the mint context
    let mint_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.ownership_token_mint.to_account_info(),
            to: ctx.accounts.user_ownership_token_account.to_account_info(),
            authority: ctx.accounts.fund.to_account_info(), // Fund is the authority
        },
        signer_seeds,
    );

    // Mint 1:1 ratio for now
    token::mint_to(mint_ctx, amount)?;

    msg!(
        "Deposited {} tokens into fund '{}', minted {} ownership tokens",
        amount,
        ctx.accounts.fund.name,
        amount
    );

    Ok(())
}
