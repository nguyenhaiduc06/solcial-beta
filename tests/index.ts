import * as anchor from "@coral-xyz/anchor";
import { Solcial } from "../target/types/solcial";

anchor.setProvider(anchor.AnchorProvider.env());

export const program = anchor.workspace.Solcial as anchor.Program<Solcial>;
export const user = (program.provider as anchor.AnchorProvider).wallet;
