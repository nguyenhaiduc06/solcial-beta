use anchor_lang::prelude::*;

use crate::state::like::*;

pub fn create_like(ctx: Context<CreateLike>, post: Pubkey) -> Result<()> {
    let like = &mut ctx.accounts.like;
    let user = &ctx.accounts.user;

    like.user = *user.key;
    like.post = post;
    like.bump = *ctx.bumps.get("like").unwrap();

    Ok(())
}

pub fn delete_like(_ctx: Context<DeleteLike>) -> Result<()> {
    Ok(())
}
