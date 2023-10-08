import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solcial } from "../target/types/solcial";
import { PublicKey } from "@solana/web3.js";
import { expect, use } from "chai";

describe("solcial", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Solcial as Program<Solcial>;
  const user = (program.provider as anchor.AnchorProvider).wallet;

  it("can create a new profile!", async () => {
    const name = "Name",
      username = "username",
      bio = "Biography 🌲";

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

    expect(profile.name).to.equal(name);
    expect(profile.username).to.equal(username);
    expect(profile.bio).to.equal(bio);
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

    expect(profile.name).to.equal(newName);
    expect(profile.username).to.equal(newUsername);
    expect(profile.bio).to.equal(newBio);
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

    expect(profile).to.be.null;
  });
});
