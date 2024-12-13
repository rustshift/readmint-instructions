use anchor_lang::prelude::*;

declare_id!("GNShSASPSNj4G9tBoeDtYFrpFhPmm1hjNgtqfyYcgg9P");

#[program]
pub mod readmint {
    use super::*;

    pub fn create_book(
        ctx: Context<CreateBook>,
        reward: u64,
        total_pages: u64,
        title: String,
        _aname: String,
    ) -> Result<()> {
        let book = &mut ctx.accounts.book;
        book.author = ctx.accounts.author.key();
        book.total_pages = total_pages;
        book.title = title;
        book.reward = reward;
        Ok(())
    }

    pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.token_balance = 0;
        Ok(())
    }

    pub fn update_user_book(
        ctx: Context<UpdateUserBook>,
        pages: u64,
        _title: String,
        _aname: String,
    ) -> Result<()> {
        let user_book = &mut ctx.accounts.user_book;
        let user = &mut ctx.accounts.user_pda;
        let book = &mut ctx.accounts.book;
        if user_book.current_page + pages > user_book.total_pages {
            user_book.current_page = user_book.total_pages;
            user.token_balance += book.reward;
            msg!("Book completed!");
        } else {
            user_book.current_page += pages;
            msg!(
                "User progressed {} pages. Current page: {}",
                pages,
                user_book.current_page
            );
        }

        Ok(())
    }

    pub fn add_book_to_user(
        ctx: Context<CreateUserBook>,
        _title: String,
        _aname: String,
    ) -> Result<()> {
        let user = &mut ctx.accounts.user_pda;
        let book = &mut ctx.accounts.book;

        let user_book = &mut ctx.accounts.user_book;
        user_book.user = user.key();
        user_book.book = book.key();
        user_book.current_page = 0;
        user_book.total_pages = book.total_pages;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title:String,aname:String)]
pub struct UpdateUserBook<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        seeds=[b"book",owner.key().as_ref()],
        bump
    )]
    pub user_pda: Box<Account<'info, User>>,

    #[account(
        seeds=[title.as_bytes(), aname.as_bytes()],
        bump
    )]
    pub book: Account<'info, Book>,

    #[account(
        mut,
        seeds = [user_pda.key().as_ref(), book.key().as_ref()],
        bump
    )]
    pub user_book: Box<Account<'info, UserBook>>,
}

#[derive(Accounts)]
#[instruction(title:String,aname:String)]
pub struct CreateUserBook<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        seeds=[b"book",owner.key().as_ref()],
        bump
    )]
    pub user_pda: Account<'info, User>,

    #[account(
        seeds=[title.as_bytes(), aname.as_bytes()],
        bump
    )]
    pub book: Account<'info, Book>,

    #[account(
        init,
        payer = owner,
        space = 8 + UserBook::INIT_SPACE,
        seeds = [user_pda.key().as_ref(), book.key().as_ref()],
        bump
    )]
    pub user_book: Box<Account<'info, UserBook>>,
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        init,
        payer=owner,
        space=8+User::INIT_SPACE,
        seeds=[b"book",owner.key().as_ref()],
        bump
    )]
    pub user: Box<Account<'info, User>>,
}

#[derive(Accounts)]
#[instruction(title: String, aname: String)]
pub struct CreateBook<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        init,
        payer = author,
        space = 8 + Book::INIT_SPACE,
        seeds = [title.as_bytes(), aname.as_bytes()],
        bump,
    )]
    pub book: Box<Account<'info, Book>>,
}

#[account]
#[derive(InitSpace)]
pub struct Book {
    pub author: Pubkey,
    #[max_len(32)]
    pub title: String,
    pub total_pages: u64,
    pub reward: u64,
}

#[account]
#[derive(InitSpace)]
pub struct User {
    pub token_balance: u64,
}

#[account]
#[derive(InitSpace)]
pub struct UserBook {
    pub user: Pubkey,
    pub book: Pubkey,
    pub current_page: u64,
    pub total_pages: u64,
}
