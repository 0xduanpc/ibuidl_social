import * as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";

export async function createTokenMintAccount(wallet: anchor.Wallet) {
  const [splTokenPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint_v9")],
    program.programId
  );
  return [
    splTokenPda,
    await program.methods.createTokenMintAccount().accounts({}).rpc(),
  ];
}
