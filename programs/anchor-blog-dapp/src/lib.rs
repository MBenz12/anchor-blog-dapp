use anchor_lang::prelude::*;

declare_id!("56Wip2hmjkjVLAzqN8x7iiSc4hgUVL977pRFEaewo6CY");

pub const BLOG_SEED_PREFIX: &str = "blog";
pub const POST_SEED_PREFIX: &str = "post";

#[program]
pub mod anchor_blog_dapp {
    use super::*;

    pub fn initialize_blog(
        ctx: Context<InitializeBlog>, 
        blog: Blog
    ) -> Result<()> {
        ctx.accounts.blog_account.set_inner(blog);
        Ok(())
    }

    pub fn create_post(ctx: Context<CreatePost>, post: Post) -> Result<()> {
        if (post.title.len() > 20) || (post.content.len() > 100) {
            return Err(ErrorCode::InvalidContentOrTitle.into());
        }
        ctx.accounts.post_account.set_inner(post);
        // ctx.accounts.blog_account.post_count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeBlog<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [
            BLOG_SEED_PREFIX.as_bytes(),
            authority.key.as_ref(),
        ],
        bump,
        space = Blog::LEN
    )]
    pub blog_account: Account<'info, Blog>,
    
    /// CHECK:    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(post: Post)]
pub struct CreatePost<'info> {
    #[account(mut)]
    pub blog_account: Account<'info, Blog>,

    #[account(
        init,
        seeds = [
            POST_SEED_PREFIX.as_bytes(),
            blog_account.key().as_ref(),
            post.slug.as_ref(),
        ],
        bump,
        payer = authority,
        space = Post::LEN,
    )]
    pub post_account: Account<'info, Post>,

    /// CHECK:
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Blog {
    pub authority: Pubkey,
    pub bump: u8,
    pub post_count: u8,
}

impl Blog {
    const LEN: usize = 8 + 32 + 1 + 1;
}

#[account]
pub struct Post {
    pub author: Pubkey,
    pub slug: String,
    pub title: String,
    pub content: String,
}

impl Post {
    const LEN: usize = 8 + 32 + (4 + 10) + (4 + 20) + (4 + 100);
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Content or Title.")]
    InvalidContentOrTitle
}