pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

// use constants::*;
use instructions::*;
use state::*;

declare_id!("G6MxBUWK1gvoQ3YGxeHZ6jCStN9mMcBE1zEzqMBBuTwe");

#[program]
pub mod funds {
    use super::*;

    pub fn create_fund(ctx: Context<CreateFund>, fund_name: String, id: u64) -> Result<()> {
        create_fund::create_fund_handler(ctx, fund_name, id)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        deposit::deposit_handler(ctx, amount)
    }
}
