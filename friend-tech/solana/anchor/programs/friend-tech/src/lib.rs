#![allow(clippy::result_large_err)]

use instructions::*;
mod instructions;
mod state;
use anchor_lang::prelude::*;

declare_id!("GcdUTgTLJ5TFQpD5r2Da8ogVnSCr8XBpmUxYdU2nzdK");

#[program]
pub mod friend_tech {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        handle_initialize(ctx)
    }

    pub fn issue_key(ctx: Context<IssueKey>, bump: u8, username: String) -> Result<()> {
        handle_issue_key(ctx, bump, username)
    }

    pub fn buy_key(ctx: Context<TransactHoldings>, k: u64) -> Result<()> {
        handle_buy_key(ctx, k)
    }

    pub fn sell_key(ctx: Context<TransactHoldings>, vault_bump: u8, k: u64) -> Result<()> {
        handle_sell_key(ctx, vault_bump, k)
    }
}
