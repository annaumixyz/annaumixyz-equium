//! Admin-only, one-way. Renounces the admin key.

use anchor_lang::prelude::*;

use crate::errors::AnnaumixyzError;
use crate::RenounceAdmin;

pub fn handler(ctx: Context<RenounceAdmin>) -> Result<()> {
    let cfg = &mut ctx.accounts.config;
    require!(!cfg.admin_renounced, AnnaumixyzError::AdminRenounced);
    cfg.admin_renounced = true;
    cfg.admin = Pubkey::default();
    Ok(())
}
