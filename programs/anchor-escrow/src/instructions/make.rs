use anchor_lang::prelude::*;

use crate::Escrow;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}
};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = Escrow::DISCRIMINATOR.len() + Escrow::INIT_SPACE,
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Make<'info> {
    // initializes the data fields in the escrow account
    pub fn init_escrow(&mut self, seed:u64, receive: u64, bumps: &MakeBumps) -> Result<()> {
        self.escrow.maker = self.maker.key();
        self.escrow.mint_a = self.mint_a.key();
        self.escrow.mint_b = self.mint_b.key();
        self.escrow.seeds = seed;
        self.escrow.receive = receive;
        self.escrow.bump = bumps.escrow;

        Ok(())
    }

    pub fn deposit(&mut self, deposit:u64) -> Result<()> {
        let cpi_context = CpiContext::new(
            self.token_program.to_account_info(),
            TransferChecked {
                from: self.maker_ata_a.to_account_info(),
                to: self.vault.to_account_info(),
                mint: self.mint_a.to_account_info(),
                authority: self.maker.to_account_info(),
            });

        transfer_checked(cpi_context, deposit, self.mint_a.decimals)
    }
}