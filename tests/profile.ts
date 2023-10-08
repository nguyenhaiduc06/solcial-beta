import * as anchor from "@coral-xyz/anchor";
import { user, program } from ".";
import { PublicKey } from "@solana/web3.js";
import { assert } from "chai";

async function createProfile(name, username, bio) {
  const [profilePDA, _] = PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("profile"), user.publicKey.toBuffer()],
    program.programId
  );

  await program.methods
    .createProfile(name, username, bio)
    .accounts({
      user: user.publicKey,
      profile: profilePDA,
    })
    .rpc();

  const profile = await program.account.profile.fetch(profilePDA);

  return profile;
}

describe("profile", () => {
  it("cannot create a new profile with name longer than 20 characters", async () => {
    try {
      await createProfile("x".repeat(21), "username", "bio");
      assert.fail(
        "still can create a new profile with name longer than 20 characters"
      );
    } catch (error) {
      assert.equal(error.error.errorCode.code, "TooLong");
    }
  });

  it("cannot create a new profile with username longer than 20 characters", async () => {
    try {
      await createProfile("name", "x".repeat(21), "bio");
      assert.fail(
        "still can create a new profile with username longer than 20 characters"
      );
    } catch (error) {
      assert.equal(error.error.errorCode.code, "TooLong");
    }
  });

  it("cannot create a new profile with bio longer than 50 characters", async () => {
    try {
      await createProfile("name", "username", "x".repeat(51));
      assert.fail(
        "still can create a new profile with bio longer than 50 characters"
      );
    } catch (error) {
      assert.equal(error.error.errorCode.code, "TooLong");
    }
  });

  it("can create a new profile!", async () => {
    const name = "Name",
      username = "username",
      bio = "Biography 🌲";

    const profile = await createProfile(name, username, bio);

    assert.equal(profile.name, name);
    assert.equal(profile.username, username);
    assert.equal(profile.bio, bio);
  });

  it("can update profile!", async () => {
    const newName = "New name",
      newUsername = "newusername",
      newBio = "New biography 🌲";

    const [profilePDA, _] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("profile"), user.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .updateProfile(newName, newUsername, newBio)
      .accounts({
        user: user.publicKey,
        profile: profilePDA,
      })
      .rpc();

    const profile = await program.account.profile.fetch(profilePDA);

    assert.equal(profile.name, newName);
    assert.equal(profile.username, newUsername);
    assert.equal(profile.bio, newBio);
  });

  it("can delete profile", async () => {
    const [profilePDA, _] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("profile"), user.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .deleteProfile()
      .accounts({
        user: user.publicKey,
        profile: profilePDA,
      })
      .rpc();

    const profile = await program.account.profile.fetchNullable(profilePDA);

    assert.equal(profile, null);
  });
});
