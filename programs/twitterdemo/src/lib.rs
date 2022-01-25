use anchor_lang::prelude::*;
declare_id!("FwwQjkYuFJdf1iJuDVK9MDx1NFDA7uior35jPg88VhQu");

#[program]
pub mod twitterdemo {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.tweets = 0;
        Ok(())
    }
    pub fn create_tweet(ctx: Context<CreateTweet>, tweet_content : String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        let timestamp = Clock::get().unwrap().unix_timestamp;
        let commentitem = CommentStruct{
            tweet_comment_time : timestamp.to_string(),
            tweet_comment : "Hello".to_string(),
            tweet_comment_user_address : *user.to_account_info().key,
        };
        let item = ItemStruct {
            tweet_id : base_account.tweets+1,
            tweet_time  :timestamp.to_string(),
            tweet_content : tweet_content.to_string(),
            tweet_likes : 0,
            comment_list : vec![commentitem],
            user_address: *user.to_account_info().key,
        };
        base_account.tweet_list.push(item);
        base_account.tweets+=1;
        Ok(())
    }
    pub fn add_like(ctx : Context<AddLike>, id : String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        let st = id.to_string();
        let size = st.parse::<i32>().unwrap();
        let index: usize = size as usize;
        base_account.tweet_list[index-1].tweet_likes+=1;
        Ok(())
    }
    pub fn add_comment(ctx : Context<AddComment>, comment : String, id : String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        let st = id.to_string();
        let size = st.parse::<i32>().unwrap();
        let index: usize = size as usize;
        let timestamp = Clock::get().unwrap().unix_timestamp;
        let item = CommentStruct {
            tweet_comment_time : timestamp.to_string(),
            tweet_comment : comment.to_string(),
            tweet_comment_user_address : *user.to_account_info().key,
        };
        base_account.tweet_list[index-1].comment_list.push(item);
        base_account.comment+=1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct CreateTweet<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddLike<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct AddComment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CommentStruct {
    pub tweet_comment_time : String,
    pub tweet_comment : String,
    pub tweet_comment_user_address: Pubkey,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub tweet_id : u64,
    pub tweet_time : String,
    pub tweet_content: String,
    pub tweet_likes : u64,
    pub comment_list : Vec<CommentStruct>,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub tweets: u64,
    pub tweet_list: Vec<ItemStruct>,
    pub comment : u64,
}