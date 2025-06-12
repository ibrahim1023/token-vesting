pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2PgGzi9yVMzD9kE5aATrgqswSsav7jYsDZWEPxJTxcpZ");

#[program]
pub mod vesting {
    use super::*;

    pub fn create_vesting_account(
        ctx: Context<CreateVestingAccount>,
        company_name: String,
    ) -> Result<()> {
        instructions::create_vesting::create_vesting_account(ctx, company_name)?;
        Ok(())
    }

    pub fn create_employee_vesting(
        ctx: Context<CreateEmployeeAccount>,
        start_time: i64,
        end_time: i64,
        total_amount: i64,
        cliff_time: i64,
    ) -> Result<()> {
        instructions::create_employee_vesting(ctx, start_time, end_time, total_amount, cliff_time)
    }

    pub fn claim_tokens(ctx: Context<ClaimTokens>, company_name: String) -> Result<()> {
        instructions::claim_tokens(ctx, company_name)
    }
}
