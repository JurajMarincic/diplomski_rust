use anchor_lang::prelude::*;
use std::ops::DerefMut;

declare_id!("F75bTjnaqScc9VZz6p5dKxFyxdBNQ48g7UURVZCwTSyH");

#[program]
pub mod ticket_system {
    use super::*;

    // Create a new ticket on the chain
    pub fn create_ticket(
        ctx: Context<CreateTicket>, 
        id: String,
        name: String, 
        departure_time: String,
        arrival_time: String,
        price: f64
    ) -> Result<()> {
        let fee_in_lamports = (price * 1_000_000_000.0) as u64;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.fee_collector.key(),
            fee_in_lamports,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.fee_collector.to_account_info(),
            ],
        )?;

        let ticket = &mut ctx.accounts.ticket;

        ticket.id = id;
        ticket.name = name;
        ticket.departure_time = departure_time;
        ticket.arrival_time = arrival_time;
        ticket.price = price;
        ticket.bump = ctx.bumps.ticket;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateTicket<'info> {
    #[account(
        init,
        payer = user,
        space = Ticket::SIZE,
        seeds = [b"ticket"],
        bump
    )]
    ticket: Account<'info, Ticket>,

    #[account(mut)]
    user: Signer<'info>,
    
    /// CHECK: This is safe because we're just transferring lamports to this account
    #[account(mut)]
    fee_collector: AccountInfo<'info>,

    system_program: Program<'info, System>,
}

#[account]
pub struct Ticket {
    pub id: String,
    pub name: String,
    pub departure_time: String,
    pub arrival_time: String,
    pub price: f64,
    pub bump: u8,
}

impl Ticket {
    // Size calculation:
    // 8 (discriminator) + 4 + 64 (max length for id) + 4 + 100 (max length for name) +
    // 4 + 64 (max length for departure_time) + 4 + 64 (max length for arrival_time) +
    // 8 (price as f64) + 1 (bump)
    pub const SIZE: usize = 8 + (4 + 64) + (4 + 64) + (4 + 64) + (4 + 64) + 8 + 1;
}

#[error_code]
pub enum ErrorCode {
    #[msg("Failed to create the ticket.")]
    TicketCreationFailed,
}