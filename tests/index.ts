import * as anchor from "@coral-xyz/anchor";
import { Solcial } from "../target/types/solcial";

anchor.setProvider(anchor.AnchorProvider.env());

import profile from "./profile";
import post from "./post";

export const program = anchor.workspace.Solcial as anchor.Program<Solcial>;
export const user = (program.provider as anchor.AnchorProvider).wallet;

describe("Solcial tests", () => {
  describe("profile", () => profile());
  describe("post", () => post());
});
