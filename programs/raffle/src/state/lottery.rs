use anchor_lang::{prelude::*};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum LotteryStatus {
    Opened,
    Closed,
    Completed,
}

#[account]
#[derive(Debug)]
pub struct Lottery {
    /// current lottery status
    pub status: LotteryStatus,

    /// nft mintkey
    pub nft_mint: Pubkey,

    /// winner ticket
    pub winner_ticket: Pubkey,

    /// owner of the nft
    pub owner: Pubkey,

    /// escrow that holds owner's nft and send it to the winner
    pub escrow: Pubkey,

    /// vault that holds the sol of the ticket price
    pub vault: Pubkey,

    /// open date
    pub start_date: Option<i64>,

    /// close date
    pub end_date: i64,

    /// ticket numbers
    pub ticket_numbers: u64,

    /// remain tickets
    pub remain_tickets: u64,

    /// limit tickets
    pub limit_tickets: u64,

    /// winners
    pub winners: u64,
}

impl Lottery {
    pub const LEN: usize = 8 + 1 + 32 + 32 + 32 + 32 + 32 + 9 + 8 + 8 + 8 + 8 + 8;
}