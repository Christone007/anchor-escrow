use anchor_lang::prelude::*;

use crate::Escrow;
use anchor_spl::{
    SystemAccount,
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked,
    },
};

// derive[Accounts] and include all the accounts required
#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    // if taker does not have an ATA to receive token A, create it
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    
    // taker's ATA for token B would be debited so maker can be credited
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    // ensure maker can receive token B into an ATA
     #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    // the mints of the tokens being passed around
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    // Close the escrow after everything and return rent to maker
    #[account(
        mut,
        close = maker,
        has_one = mint_a,
        has_one = maker,
        seeds = [b"escrow", maker.key().as_ref(), &escrow.seeds.to_le_bytes()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    // The vault to be emptied into the takers ata a
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // other programs
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

// impl fn transfer, withdraw and close
impl<'info> Take<'info> {
    pub fn deposit(&mut self, deposit:u64) -> Result<()> {
        // runs a Cpi to transfer from taker_ata_b to maker_ata_b
        // ensure the amount equals the receive amount set in the escrow
        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            TransferChecked {
                from: self.taker_ata_b.to_account_info(),
                to: self.maker_ata_b.to_account_info(),
                mint: self.mint_b.to_account_info(),
                authority: self.taker.to_account_info(),
            }
        );

        transfer_checked(cpi_ctx, deposit, self.mint_b.decimals)
    }

    pub fn withdraw(&mut self, receive:u64) -> Result<()> {
        // runs a Cpi to transfer from vault(holding token a) to taker_ata_a
        // ensures everything in vault is transferred
        Ok(())
    }

    pub fn close_vault(&mut self) -> Result<()> {
        // closes the vault and sends the rent to the taker since he signed this transaction

        Ok(())
    }
}