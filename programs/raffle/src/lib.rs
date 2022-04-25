pub mod state;
pub mod utils;
mod processor;
pub mod error;

use anchor_lang::{prelude::*};
use anchor_spl::token::{ Token, TokenAccount };

declare_id!("EpNpQFNZMcUJjvmUdYi8KvrGK6wqeu8RzhxdUGMypeNh");

#[program]
pub mod raffle {
    use super::*;

    pub fn upload(
        _ctx: Context<UploadAccount>,
        start_date: Option<i64>, 
        end_date: i64
    ) -> Result<()> {
        _ctx.accounts.process(
            *_ctx.bumps.get("escrow").unwrap(),
            *_ctx.bumps.get("vault").unwrap(),
            start_date,
            end_date
        )
    }

    pub fn close_lottery(
        _ctx: Context<CloseAccount>,
        tickets: Vec<Pubkey>,
    ) -> Result<()> {
        _ctx.accounts.process(
            tickets
        )
    }

    pub fn claim(
        _ctx: Context<ClaimAccount>,
    ) -> Result<()> {
        _ctx.accounts.process(
            *_ctx.bumps.get("escrow").unwrap(),
        )
    }

    // pub fn create_vault (_ctx: Context<VaultAccount>, _bump_vault: u8) -> Result<()> {
    //     Ok(())
    // }
}

// #[derive(Accounts)]
// #[instruction(bump: u8)]
// pub struct VaultAccount<'info> {
//     #[account(init, seeds = [b"vault".as_ref()], bump, payer = admin, space = 9)]
//     pub vault: Account<'info, Vault>,
//     #[account(mut)]
//     pub admin: Signer<'info>,
//     pub system_program: Program<'info, System>
// }

// #[account]
// pub struct Vault {
//     pub bump_vault: u8
// }

#[derive(Accounts)]
pub struct UploadAccount<'info> {
    /// lottery info
    #[account(
        init, 
        seeds = [
            utils::LOTTERY_PREFIX.as_bytes(), 
            user.key().as_ref(), 
            nft_mint.key().as_ref(),
        ], 
        bump, 
        payer = user, 
        space = state::Lottery::LEN
    )]
    pub lottery: Box<Account<'info, state::Lottery>>,

    /// the owner of the lottery nft
    #[account(mut)]
    pub user: Signer<'info>,

    /// user nft account
    /// should be 'spl_token' account should be passed
    /// CHECK: it's alright
    #[account(mut)]
    pub user_nft_account: AccountInfo<'info>,

    /// nft mintkey to bet
    /// CHECK: it's alright
    pub nft_mint: AccountInfo<'info>,

    /// will hold nft
    /// PDA: [LOTTERY_ESCROW_PREFIX, owner_pubkey, lottery_pubkey]
    /// CHECK: it's alright
    #[account(
        mut, 
        seeds = [
            utils::LOTTERY_ESCROW_PREFIX.as_bytes(), 
            user.key().as_ref(), 
            lottery.key().as_ref(),
        ], 
        bump
    )]
    pub escrow: AccountInfo<'info>,

    /// will hold the sol of the raffles tickets price
    /// CHECK: it's alright
    #[account(
        mut, 
        seeds = [
                utils::LOTTERY_ESCROW_VAULT_PREFIX.as_bytes(),
                escrow.key().as_ref(),
        ],
        bump,
    )]
    pub vault: AccountInfo<'info>,

    pub rent_sysvar: Sysvar<'info, Rent>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

/// close the lottery
/// choose the winner ticket
/// check if the time passed the end date
#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(mut)]
    pub lottery: Box<Account<'info, state::Lottery>>,

    pub clock_sysvar: Sysvar<'info, Clock>,
}

/// Perform claim. 
/// Send the nft to the winner
/// Check if the user is the winner by comparing metadata
#[derive(Accounts)]
pub struct ClaimAccount<'info> {
    #[account(
        mut, 
        seeds = [
            utils::LOTTERY_PREFIX.as_bytes(), 
            owner.key().as_ref(), 
            nft_mint.key().as_ref(),
        ], 
        bump, 
        has_one = escrow, 
        has_one = owner, 
        has_one = nft_mint,
        has_one = vault,
        has_one = winner_ticket,
    )]
    pub lottery: Box<Account<'info, state::Lottery>>,

    /// user who claims
    /// must be the winner
    /// CHECK: it's alright
    #[account(mut)]
    pub user: Signer<'info>,

    /// owner of the nft
    /// CHECK: it's alright
    #[account(mut)]
    pub owner: UncheckedAccount<'info>,

    /// the nft mint key
    /// CHECK: it's alright
    #[account(mut)]
    pub nft_mint: UncheckedAccount<'info>,

    /// the winner ticket
    /// must be equal to the lottery winner ticket
    /// CHECK: it's alright
    pub winner_ticket: UncheckedAccount<'info>,

    /// the ticket token account
    #[account(mut)]
    pub ticket_token_account: Account<'info, TokenAccount>,

    /// user's receive_token_account to receive the nft
    #[account(mut)]
    pub receive_token_account: Account<'info, TokenAccount>,
    
    /// escrow that holds nft
    /// CHECK: it's alright
    #[account(
        mut, 
        seeds = [
            utils::LOTTERY_ESCROW_PREFIX.as_bytes(), 
            lottery.owner.as_ref(), 
            lottery.key().as_ref(),
        ], 
        bump
    )]
    pub escrow: UncheckedAccount<'info>,

    /// vault that holds the sol
    /// CHECK: it's alright
    #[account(
        mut, 
        seeds = [
            utils::LOTTERY_ESCROW_VAULT_PREFIX.as_bytes(),
            escrow.key().as_ref(),            
        ],
        bump
    )]
    pub vault: UncheckedAccount<'info>,

    pub clock_sysvar: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}