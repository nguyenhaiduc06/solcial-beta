import * as anchor from "@coral-xyz/anchor";
import { user, program } from ".";
import { Keypair, PublicKey } from "@solana/web3.js";
import { createPost } from "./post";
import { assert } from "chai";

describe("like", () => {
  const postKeypair = Keypair.generate();

  const [likePDA, bump] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("like"),
      user.publicKey.toBuffer(),
      postKeypair.publicKey.toBuffer(),
    ],
    program.programId
  );

  it("can like a post", async () => {
    await createPost(postKeypair, "Content", "tag");
    await program.methods
      .createLike(postKeypair.publicKey)
      .accounts({
        user: user.publicKey,
        like: likePDA,
      })
      .rpc();

    const like = await program.account.like.fetchNullable(likePDA);

    assert.equal(like.user.toString(), user.publicKey.toString());
    assert.equal(like.post.toString(), postKeypair.publicKey.toString());
    assert.equal(like.bump, bump);
  });

  it("can unlike a post", async () => {
    await program.methods
      .deleteLike()
      .accounts({
        user: user.publicKey,
        like: likePDA,
      })
      .rpc();

    const like = await program.account.like.fetchNullable(likePDA);

    assert.equal(like, null);
  });
});
