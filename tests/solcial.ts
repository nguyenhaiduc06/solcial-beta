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
    const [profilePDA, _] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("profile"), user.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .createProfile("username")
      .accounts({
        user: user.publicKey,
        profile: profilePDA,
      })
      .rpc();

    const profile = await program.account.profile.fetch(profilePDA);

    expect(profile.username).to.equal("username");
  });

  it("can update profile!", async () => {
    const [profilePDA, _] = PublicKey.findProgramAddressSync(
      [anchor.utils.bytes.utf8.encode("profile"), user.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .updateProfile("newusername")
      .accounts({
        user: user.publicKey,
        profile: profilePDA,
      })
      .rpc();

    const profile = await program.account.profile.fetch(profilePDA);

    expect(profile.username).to.equal("newusername");
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
