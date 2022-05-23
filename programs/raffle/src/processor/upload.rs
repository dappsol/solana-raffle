use crate:: {error, id, state, utils, UploadAccount};
use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};
use anchor_spl::token;

impl<'info> UploadAccount<'info> {
    pub fn process(
        &mut self,
        escrow_bump: u8,
        vault_bump: u8,
        start_date: Option<UnixTimestamp>,
        end_date: UnixTimestamp,
        ticket_numbers: u64,
        limit_tickets: u64,
        winners: u64,
    ) -> Result<()> {
        self.lottery.status = state::LotteryStatus::Opened;

        self.lottery.nft_mint = self.nft_mint.key().clone();

        self.lottery.owner = self.user.key().clone();

        self.lottery.escrow = self.escrow.key().clone();

        self.lottery.vault = self.vault.key().clone();

        self.lottery.start_date = start_date;
        self.lottery.end_date = end_date;
        self.lottery.ticket_numbers = ticket_numbers;
        self.lottery.remain_tickets = ticket_numbers;
        self.lottery.limit_tickets = limit_tickets;
        self.lottery.winners = winners;

        // create 'spl' escrow account to hold user's nft
        utils::sys_create_account(
            &self.user.to_account_info(),
            &self.escrow.to_account_info(),
            self.rent_sysvar.minimum_balance(token::TokenAccount::LEN),
            token::TokenAccount::LEN,
            &token::Token::id(),
            &[
                utils::LOTTERY_ESCROW_PREFIX.as_bytes(),
                self.user.key().as_ref(),
                self.lottery.key().as_ref(),
                &[escrow_bump],
            ]
        )?;

        // initialize escrow 'spl_token' account
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::InitializeAccount {
            account: self.escrow.to_account_info(),
            mint: self.nft_mint.to_account_info(),
            authority: self.escrow.to_account_info(),
            rent: self.rent_sysvar.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &[]);
        token::initialize_account(cpi_ctx)?;
        
        // transfer nft to spl_token escrow
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::Transfer {
            from: self.user_nft_account.to_account_info(),
            to: self.escrow.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &[]);
        token::transfer(cpi_ctx, 1)?;

        // initialise vault account
        utils::sys_create_account(
            &self.user.to_account_info(), 
            &self.vault.to_account_info(), 
            self.rent_sysvar.
                minimum_balance(utils::ORDER_ESCROW_NATIVE_SIZE), 
            utils::ORDER_ESCROW_NATIVE_SIZE, 
            &id(), 
            &[
                utils::LOTTERY_ESCROW_VAULT_PREFIX.as_bytes(),
                self.escrow.key().as_ref(),
                &[vault_bump],
            ],
        )?;

        // send 100sol to the vault
        utils::sys_transfer(
            &self.user.to_account_info(),
            &self.vault.to_account_info(), 
            1 * utils::LAMPORTS_PER_SOL,
            &[],
        )?;

        // check end date
        if self.clock_sysvar.unix_timestamp >= self.lottery.end_date {
            return Err(error::ErrorCode::ExpireDateInThePast.into());
        }

        // check the start date
        if let Some(start_date) = self.lottery.start_date {
            // if self.clock_sysvar.unix_timestamp > start_date {
            //     return Err(error::ErrorCode::StartDateInThePast.into());
            // }

            if start_date >= self.lottery.end_date {
                return Err(error::ErrorCode::ExpireDateInThePast.into())
            }
        }

        Ok(())
    }
}