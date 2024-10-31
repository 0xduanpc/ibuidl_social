import * as anchor from "@coral-xyz/anchor";
import { program } from "./wallet";

export async function createTweet(
  wallet: anchor.Wallet,
  body: string
): Promise<[anchor.web3.PublicKey, string]> {
  const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), wallet.publicKey.toBuffer()],
    program.programId
  );

  const profile = await program.account.ibuidlProfile.fetch(profilePda);

  const [tweetPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("tweet"),
      profilePda.toBuffer(),
      Buffer.from(`${profile.tweetCount + 1}`),
    ],
    program.programId
  );

  // tweetPda需要计算（先查profile，再+1）得出，需要手动传
  return [
    tweetPda,
    await program.methods
      .createTweet(body)
      .accounts({ authority: wallet.publicKey, tweet: tweetPda })
      .rpc(),
  ];
}

export async function getTweet(
  wallet: anchor.Wallet,
  tweetPda: anchor.web3.PublicKey
) {
  return await program.account.ibuidlTweet.fetch(tweetPda);
}

export async function createLike(
  wallet: anchor.Wallet,
  tweetPda: anchor.web3.PublicKey
) {
  const tweet = await program.account.ibuidlTweet.fetch(tweetPda);

  // like的seeds都为常量，anchor可以自动计算，accounts里不需要再传了
  // wallet.ts中使用了anchor.setProvider(provider)
  // 所以anchor默认会把Anchor.toml文件中[provider]设置的wallet放到singers中
  // 但createlike需要使用vistorWallet，所以需要显式的传入了
  return await program.methods
    .createLike()
    .accounts({
      tweet: tweetPda,
      authority: wallet.publicKey,
      authorWallet: tweet.author,
    })
    .signers([wallet.payer])
    .rpc();
}
