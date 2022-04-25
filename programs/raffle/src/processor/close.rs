use crate::{error, state, utils, CloseAccount};
use anchor_lang::prelude::*;
use anchor_spl::token;

impl<'info> CloseAccount<'info> {
    pub fn process(
        &mut self,
        tickets: Vec<Pubkey>,
    ) -> Result<()> {
        let rand = utils::random(self.clock_sysvar.slot as u32);
        let length = tickets.len();
        let index = rand as usize % length;
        self.lottery.winner_ticket = tickets[index];

        if self.lottery.status != state::LotteryStatus::Opened {
            return Err(error::ErrorCode::LotteryNotOpen.into());
        }
        
        self.lottery.status = state::LotteryStatus::Closed;
        
        Ok(())
    }
}