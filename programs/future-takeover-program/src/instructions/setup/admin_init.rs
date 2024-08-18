use anchor_lang::prelude::*;

use crate::{
    constant::admin_wallet as ADMIN,
    state::AdminProfile, 
    errors::TakeoverError
};

#[derive(Accounts)]
#[instruction(username: String)]
pub struct AdminInit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub new_admin: SystemAccount<'info>,
    #[account(
        init,
        payer = owner,
        space = AdminProfile::INIT_SPACE + username.len(),
        seeds = [b"admin", new_admin.key().as_ref()],
        bump
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    pub system_program: Program<'info, System>,
}

/*
        
    Create a new Admin Ix:

    Some security check:
    - Check if the account that is initializing the admin is the admin of the entire protocol.
    - Make sure the admin profile we're creating is not for the admin of the entire protocol, that might be a security issues.
    - Save the Time of initialization to render it useless for the first 16h of initialization.

    What the Instruction does:
    - Initialize the new admin account with the username (so we can monitor who are the admin
    account atm in an easy way) and the publickey of the new admin.

*/

impl<'info> AdminInit<'info> {
    pub fn initialize_admin_profile(&mut self, username: String, bump: u8) -> Result<()> {
        
        self.admin_profile.set_inner(
            AdminProfile {
                address: self.new_admin.key(),
                username,
                creation_time: Clock::get()?.unix_timestamp - 20 * 60 * 60,
                bump,
            }
        );

        Ok(())
    }
}

pub fn handler(ctx: Context<AdminInit>, username: String) -> Result<()> {
    // Make sure it's the admin of the protocol that is initializing the new admin and that the new admin is not the admin of the protocol
    // require!(ctx.accounts.owner.key() == ADMIN::id() && ctx.accounts.owner.key() != ctx.accounts.new_admin.key(), TakeoverError::Unauthorized); -> Uncomment this

    // Generate the bumps
    let bumps = ctx.bumps;

    // Initialize the new admin_profile
    ctx.accounts.initialize_admin_profile(username, bumps.admin_profile)?;

    Ok(())
}