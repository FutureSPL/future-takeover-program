use anchor_lang::prelude::*;

use crate::{
    constant::admin_wallet as ADMIN,
    state::AdminProfile, 
    errors::TakeoverError
};

#[derive(Accounts)]
pub struct AdminDelete<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub old_admin: SystemAccount<'info>,
    #[account(
        mut,
        close = owner,
        seeds = [b"admin", old_admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,
    pub system_program: Program<'info, System>,
}

/*
        
    Close an old Admin Ix:

    Some security check:
    - Check if the account that is initializing the admin is the admin of the entire protocol.

    What the Instruction does:
    - Close the old admin account.

*/

impl<'info> AdminDelete<'info> {

}

pub fn handler(ctx: Context<AdminDelete>) -> Result<()> {
    // Make sure it's the admin of the protocol that is closing the old admin
    //require!(ctx.accounts.owner.key() == ADMIN::id(), TakeoverError::Unauthorized);

    Ok(())
}