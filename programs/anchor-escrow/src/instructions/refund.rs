use anchor_lang::prelude::*;

use crate::Escrow;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked,
    },
};

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        has_one = mint_a,
        has_one = maker,
        seeds = [b"escrow", maker.key().as_ref(), &escrow.seeds.to_le_bytes()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,


    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
} 

impl<'info> Refund<'info> {
    pub fn refund_and_close_vault(&mut self) -> Result<()> {
        // transfer vault funds to maker_ata_a
        let signer_seeds = &[&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seeds.to_le_bytes(),
            &[self.escrow.bump],
        ]];

        let transfer_cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            TransferChecked {
                from: self.vault.to_account_info(),
                to: self.maker_ata_a.to_account_info(),
                mint: self.mint_a.to_account_info(),
                authority: self.escrow.to_account_info(),
            }
        ).with_signer(signer_seeds);

        transfer_checked(transfer_cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        // close escrow and transfer
        let close_accounts = CloseAccount {
            account:
        }
    }
}