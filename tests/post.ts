import { user, program } from ".";
import { Keypair } from "@solana/web3.js";
import { expect } from "chai";

export const createPost = async (postKeypair, content, tag) => {
  await program.methods
    .createPost(content, tag)
    .accounts({
      author: user.publicKey,
      post: postKeypair.publicKey,
    })
    .signers([postKeypair])
    .rpc();

  const post = await program.account.post.fetchNullable(postKeypair.publicKey);
  return post;
};

describe("post", () => {
  const postKeypair = Keypair.generate();

  it("can create a new post", async () => {
    const content = "Content",
      tag = "tag";

    const post = await createPost(postKeypair, content, tag);

    expect(post.content).to.equal(content);
    expect(post.tag).to.equal(tag);
  });

  it("can update post", async () => {
    const newContent = "New content",
      newTag = "new tag";

    await program.methods
      .updatePost(newContent, newTag)
      .accounts({
        author: user.publicKey,
        post: postKeypair.publicKey,
      })
      .rpc();

    const post = await program.account.post.fetch(postKeypair.publicKey);

    expect(post.content).to.equal(newContent);
    expect(post.tag).to.equal(newTag);
  });

  it("can delete post", async () => {
    await program.methods
      .deletePost()
      .accounts({
        author: user.publicKey,
        post: postKeypair.publicKey,
      })
      .rpc();

    const post = await program.account.post.fetchNullable(
      postKeypair.publicKey
    );

    expect(post).to.be.null;
  });
});
