use anchor_lang::prelude::*;

use crate::Escrow;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked,
    },
};

// derive[Accounts] and include all the accounts required
#[derive(Accounts)]
pub struct Take<'info> {
    pub taker: Signer<'info>,
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub escrow: Account<'info, Escrow>,
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

// impl fn transfer, withdraw and close