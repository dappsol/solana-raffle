use crate::{error, state, utils, ClaimAccount};
use anchor_lang::prelude::*;
use anchor_spl::token;

impl<'info> ClaimAccount<'info> {
    pub fn process(
        &mut self, 
        escrow_bump: u8,
    ) -> Result<()> {
        if self.lottery.status != state::LotteryStatus::Closed {
            return Err(error::ErrorCode::InvalidLotteryStatus.into());
        }

        if self.ticket_token_account.mint != self.lottery.winner_ticket {
            return Err(error::ErrorCode::InvalidTicket.into());
        }

        if self.ticket_token_account.amount != 1 {
            return Err (error::ErrorCode::NoTicket.into());
        }

        if self.user.key() != self.ticket_token_account.owner {
            return Err(error::ErrorCode::NotOwner.into());
        }

        self.lottery.status = state::LotteryStatus::Completed;

        // Transfer nft from escrow to winner
        let lottery_key = self.lottery.key();

        let signer_seeds: &[&[&[u8]]] = &[&[
            utils::LOTTERY_ESCROW_PREFIX.as_bytes(),
            self.lottery.owner.as_ref(),
            lottery_key.as_ref(),
            &[escrow_bump],
        ]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::Transfer {
            from: self.escrow.to_account_info(),
            to: self.receive_token_account.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds.clone());
        token::transfer(cpi_ctx, 1)?;

        // distribute the sol
        // let src_lamports = self.escrow.to_account_info().lamports();
        // let src_lamports = 100 * utils::LAMPORTS_PER_SOL;
        // utils::move_lamports(
        //     &self.vault.to_account_info(),
        //     &self.owner.to_account_info(),
        //     src_lamports,
        // )?;

        // Delete the vault account
        utils::delete_account(
            &self.vault.to_account_info(),
            &self.owner.to_account_info(),
        )?;

        // let vault = &self.vault.to_account_info();
        // let owner = self.owner.to_account_info();
        // **owner.try_borrow_mut_lamports()? += utils::LAMPORTS_PER_SOL;
        // **vault.lamports.borrow_mut() -= utils::LAMPORTS_PER_SOL;

        // // Delete escrow account
        // let cpi_program = self.token_program.to_account_info();
        // let cpi_accounts = token::CloseAccount {
        //     account: self.escrow.to_account_info(),
        //     destination: self.owner.to_account_info(),
        //     authority: self.escrow.to_account_info(),
        // };
        // let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        // token::close_account(cpi_ctx)?;

        if self.clock_sysvar.unix_timestamp > self.lottery.end_date {
            return Err(error::ErrorCode::LotteryIsClosed.into());
        }

        if let Some(start_date) = self.lottery.start_date {
            if start_date > self.clock_sysvar.unix_timestamp {
                return Err(error::ErrorCode::LotteryIsNotStarted.into());
            }
        }

        Ok(())
    }
}