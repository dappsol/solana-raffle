// use crate::id;
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, system_instruction},
};

pub const LAMPORTS_PER_SOL : u64 = 1_000_000_000;

pub const LOTTERY_ESCROW_PREFIX : &str = "lottery_escrow";
pub const LOTTERY_PREFIX: &str = "lottery";
pub const LOTTERY_ESCROW_VAULT_PREFIX: &str = "lottery_escrow_vault";
pub const ORDER_ESCROW_NATIVE_SIZE: usize = 0;

// wrapper of 'create_account' instruction from 'system_program' program
#[inline(always)]
pub fn sys_create_account<'a>(
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    lamports: u64,
    space: usize,
    owner: &Pubkey,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    invoke_signed(
        &system_instruction::create_account(from.key, to.key, lamports, space as u64, owner),
        &[from.clone(), to.clone()],
        &[&signer_seeds],
    )?;

    Ok(())
}

/// Move lamports from `src` to `dst` account.
#[inline(always)]
pub fn move_lamports<'a>(
    src: &AccountInfo<'a>,
    dst: &AccountInfo<'a>,
    lamports: u64,
) -> Result<()> {
    let mut src_lamports = src.try_borrow_mut_lamports()?;
    let mut dst_lamports = dst.try_borrow_mut_lamports()?;

    **src_lamports -= lamports;
    **dst_lamports += lamports;

    Ok(())
}

/// Delete `target` account, transfer all lamports to `receiver`.
#[inline(always)]
pub fn delete_account<'a>(target: &AccountInfo<'a>, receiver: &AccountInfo<'a>) -> Result<()> {
    let mut target_lamports = target.try_borrow_mut_lamports()?;
    let mut receiver_lamports = receiver.try_borrow_mut_lamports()?;

    **receiver_lamports += **target_lamports;
    **target_lamports = 0;

    Ok(())
}

// wrapper of transfer instructin from system_program program
#[inline(always)]
pub fn sys_transfer<'a>(
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    lamports: u64,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    invoke_signed(
        &system_instruction::transfer(from.key, to.key, lamports), 
        &[from.clone(), to.clone()],
        &[&signer_seeds],
    )?;

    Ok(())
}

#[inline(always)]
pub fn random(seed: u32) -> u32 {
    let a : u32 = 1103515245;
    let c : u32 = 12345;
    let m : u32 = 2 ^ 32;

    return (a * seed + c) % m;
} 