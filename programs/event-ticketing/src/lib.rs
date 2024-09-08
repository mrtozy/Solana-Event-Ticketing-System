use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, mint_to, MintTo, transfer, Transfer};


declare_id!("8k4gqaooexCmHcGoMgCYNq8To8Xkqnhq1QAbaDRKMEWc");

#[program]
mod event_ticketing {
    use super::*;

    // Yeni bir etkinlik oluşturma fonksiyonu
    pub fn create_event(ctx: Context<CreateEvent>, event_name: String, ticket_price: u64) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.name = event_name; // Etkinliğin adını ayarla
        event.ticket_price = ticket_price; // Etkinlik bilet fiyatını ayarla
        event.creator = ctx.accounts.creator.key(); // Etkinlik yaratıcısını belirle
        Ok(())
    }

    // Yeni bir bilet mint etme (oluşturma) fonksiyonu
    pub fn mint_ticket(ctx: Context<MintTicket>, ticket_id: u64) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket;
        ticket.owner = ctx.accounts.owner.key(); // Biletin sahibini belirle
        ticket.event = ctx.accounts.event.key(); // Bileti bir etkinliğe bağla
        ticket.ticket_id = ticket_id; // Biletin benzersiz ID'si aktarılıyor

        // NFT token'ı sahibinin token hesabına mint et
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.minter.to_account_info(),
            },
        );
        mint_to(cpi_ctx, 1)?; // Bir token mint et ve sahibinin hesabına gönder

        Ok(())
    }

    // Bilet satın alma fonksiyonu
    pub fn purchase_ticket(ctx: Context<PurchaseTicket>, ticket_count: u64) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket;
        let event = &ctx.accounts.event;

        // Alınacak bilet sayısına göre toplam fiyatı hesapla
        let total_price = ticket_count * event.ticket_price;
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.buyer_token_account.to_account_info(),
                to: ctx.accounts.creator_token_account.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            },
        );

        transfer(transfer_ctx, total_price)?; // Toplam fiyatı alıcının hesabından etkinlik yaratıcısının hesabına aktar

        // Bilet sahipliğini ve miktarını güncelle
        ticket.owner = ctx.accounts.buyer.key(); // Biletin yeni sahibini belirleniyor
        ticket.ticket_count += ticket_count; // Sahip olunan bilet sayısını artırılıyor
        Ok(())
    }

    // Bilet devretme fonksiyonu
    pub fn transfer_ticket(ctx: Context<TransferTicket>) -> Result<()> {
        let ticket = &mut ctx.accounts.ticket;
        ticket.owner = ctx.accounts.new_owner.key(); // Biletin yeni sahibini belirle
        Ok(())
    }

    // Bir bileti satışa çıkarma fonksiyonu
    pub fn list_ticket(ctx: Context<ListTicket>) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        listing.ticket = ctx.accounts.ticket.key(); // Listelenen biletler
        listing.seller = ctx.accounts.seller.key(); // Biletin satıcısını belirle
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    // Etkinlik bilgilerini saklayan hesap
    #[account(init, payer = creator, space = 8 + 32 + 32 + 8 + 1, seeds = [b"event", creator.key().as_ref()], bump)]
    pub event: Account<'info, Event>,
    // Etkinliği oluşturan kişi
    #[account(mut)]
    pub creator: Signer<'info>,
    // Hesapları yönetmek için sistem programı
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTicket<'info> {
    // Bilet bilgilerini saklayan hesap
    #[account(init, payer = owner, space = 8 + 32 + 32 + 8 + 8 + 1, seeds = [b"ticket", owner.key().as_ref(), event.key().as_ref()], bump)]
    pub ticket: Account<'info, Ticket>,
    // Biletin sahibi
    #[account(mut)]
    pub owner: Signer<'info>,
    // Bu biletle ilişkili etkinlik
    pub event: Account<'info, Event>,
    // Yeni token'lar oluşturmak için mint hesabı
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    // Mint edilen token'ların gönderileceği token hesabı
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    // Token'ları mint etmek için yetkili kişi
    #[account(mut)]
    pub minter: Signer<'info>,
    // Token işlemleri için kullanılan token programı
    pub token_program: Program<'info, Token>,
    // Hesapları yönetmek için sistem programı
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseTicket<'info> {
    // Satın alınan bilet
    #[account(mut)]
    pub ticket: Account<'info, Ticket>,
    // Bu biletle ilişkili etkinlik
    #[account(mut)]
    pub event: Account<'info, Event>,
    // Bileti satın alan kişi
    #[account(mut)]
    pub buyer: Signer<'info>,
    // Alıcının token hesabı
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    // Etkinlik yaratıcısının token hesabı
    #[account(mut)]
    pub creator_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferTicket<'info> {
    // Devredilen bilet
    #[account(mut)]
    pub ticket: Account<'info, Ticket>,
    // Biletin yeni sahibi
    #[account(mut)]
    pub new_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ListTicket<'info> {
    // Listeleme bilgilerini saklayan hesap
    #[account(init, payer = seller, space = 8 + 32 + 32 + 8 + 1, seeds = [b"listing", ticket.key().as_ref()], bump)]
    pub listing: Account<'info, Listing>,
    // Satışa çıkarılan bilet
    #[account(mut)]
    pub ticket: Account<'info, Ticket>,
    // Bileti satan kişi
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Event {
    pub name: String, // Etkinliğin adı
    pub ticket_price: u64, // Tek bir biletin fiyatı
    pub creator: Pubkey, // Etkinlik yaratıcısı
}

#[account]
pub struct Ticket {
    pub owner: Pubkey, // Biletin mevcut sahibi
    pub event: Pubkey, // Biletin ilişkili olduğu etkinlik
    pub ticket_id: u64, // Biletin benzersiz ID'si
    pub ticket_count: u64, // Kişinin sahibin sahip olduğu bilet sayısı
}

#[account]
pub struct Listing {
    pub ticket: Pubkey, // Satışa çıkarılan bilet
    pub seller: Pubkey, // Satıcı
}
