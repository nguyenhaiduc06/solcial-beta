import { user, program } from ".";
import { Keypair } from "@solana/web3.js";
import { expect } from "chai";

describe("comment", () => {
  const postKeypair = Keypair.generate();
  const commentKeypair = Keypair.generate();

  it("can comment on a post", async () => {
    const content = "Comment",
      tag = "tag";

    await program.methods
      .createPost(content, tag)
      .accounts({
        author: user.publicKey,
        post: postKeypair.publicKey,
      })
      .signers([postKeypair])
      .rpc();

    await program.methods
      .createComment("Comment", postKeypair.publicKey)
      .accounts({
        comment: commentKeypair.publicKey,
        author: user.publicKey,
      })
      .signers([commentKeypair])
      .rpc();

    const comment = await program.account.comment.fetch(
      commentKeypair.publicKey
    );

    expect(comment.author.toString()).to.equal(user.publicKey.toString());
    expect(comment.post.toString()).to.equal(postKeypair.publicKey.toString());
    expect(comment.content).to.equal("Comment");
  });

  it("can update a comment", async () => {
    const newContent = "New comment";

    await program.methods
      .updateComment(newContent)
      .accounts({
        comment: commentKeypair.publicKey,
        author: user.publicKey,
      })
      .rpc();

    const comment = await program.account.comment.fetch(
      commentKeypair.publicKey
    );

    expect(comment.author.toString()).to.equal(user.publicKey.toString());
    expect(comment.post.toString()).to.equal(postKeypair.publicKey.toString());
    expect(comment.content).to.equal(newContent);
  });

  it("can delete a comment", async () => {
    await program.methods
      .deleteComment()
      .accounts({
        comment: commentKeypair.publicKey,
        author: user.publicKey,
      })
      .rpc();

    const comment = await program.account.comment.fetchNullable(
      commentKeypair.publicKey
    );

    expect(comment).to.be.null;
  });
});
