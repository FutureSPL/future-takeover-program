use anchor_lang::declare_id;

// ADMIN BUFFER TO BECOME ACTIVE
pub const ADMIN_BUFFER: i64 = 16 * 60 * 60;

// TAKEOVER BUFFER TO MODIFY
pub const TAKEOVER_BUFFER: i64 = 24 * 60 * 60;

// Low FDMC: 
pub const LOW_REWARDS_BASIS_POINT: u16 = 200;
pub const LOW_TREASURY_BASIS_POINT: u16 = 550;
pub const LOW_PRESALE_BASIS_POINT: u16 = 750;

// Medium FDMC:
pub const MEDIUM_REWARDS_BASIS_POINT: u16 = 150;
pub const MEDIUM_TREASURY_BASIS_POINT: u16 = 450;
pub const MEDIUM_PRESALE_BASIS_POINT: u16 = 600;

// High FDMC:
pub const HIGH_REWARDS_BASIS_POINT: u16 = 100;
pub const HIGH_TREASURY_BASIS_POINT: u16 = 350;
pub const HIGH_PRESALE_BASIS_POINT: u16 = 500;


pub mod admin_wallet {
    use super::*;
    declare_id!("2YkGRHjwD3jqcu4ie6pL9Axpdx5AKa6KDyj8bF473Vk5");
}
