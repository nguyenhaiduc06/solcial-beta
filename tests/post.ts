import { user, program } from ".";
import { Keypair } from "@solana/web3.js";
import { expect } from "chai";

export default () => {
  it("can create a new post", async () => {
    const postKeypair = Keypair.generate();

    await program.methods
      .createPost("Hello world", "tag")
      .accounts({
        author: user.publicKey,
        post: postKeypair.publicKey,
      })
      .signers([postKeypair])
      .rpc();

    const post = await program.account.post.fetch(postKeypair.publicKey);

    expect(post.content).to.equal("Hello world");
  });

  it("can delete post", async () => {
    const postKeypair = Keypair.generate();

    await program.methods
      .createPost("Hello world", "tag")
      .accounts({
        author: user.publicKey,
        post: postKeypair.publicKey,
      })
      .signers([postKeypair])
      .rpc();

    const post = await program.account.post.fetch(postKeypair.publicKey);

    console.log(post);

    expect(post.content).to.equal("Hello world");
  });
};
