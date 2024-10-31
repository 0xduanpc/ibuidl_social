import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IbuidlSocial } from "../../target/types/ibuidl_social";

let provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.IbuidlSocial as Program<IbuidlSocial>;

export { program, provider };

export function useDefaultWallet() {
  return anchor.Wallet.local();
}

// Pubkey 56TVGTU88YtAida1D69pvLRTgUHLUo6w3o2YiFVVFRwS
export function useVisitorWallet() {
  const keypair = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array([
      203, 69, 244, 139, 155, 66, 182, 5, 95, 201, 225, 123, 148, 23, 227, 137,
      39, 125, 74, 37, 223, 58, 66, 249, 132, 42, 153, 115, 235, 82, 33, 151,
      60, 212, 153, 225, 119, 68, 117, 142, 17, 99, 78, 6, 131, 86, 128, 171,
      215, 172, 88, 245, 123, 136, 104, 36, 50, 238, 243, 36, 81, 64, 104, 37,
    ])
  );
  return new anchor.Wallet(keypair);
}
