import { publicKey } from "@metaplex-foundation/umi";
import { createProfile, getProfile } from "./api/profile";
import { createLike, createTweet, getTweet } from "./api/tweet";
import { useDefaultWallet, useVisitorWallet } from "./api/wallet";
import { createTokenMintAccount } from "./api/token";

(async () => {
  const defaultWallet = useDefaultWallet();
  const visitorWallet = useVisitorWallet();

  // const r1 = await createProfile(visitorWallet, "Bob");
  // console.log(r1);

  // const r2 = await getProfile(visitorWallet);
  // console.log(r2);

  const [pda, r3] = await createTweet(defaultWallet, "hello world");
  console.log(r3);

  const r4 = await getTweet(defaultWallet, pda);
  console.log(r4);

  const r5 = await createLike(visitorWallet, pda);
  console.log(r5);

  const r6 = await getTweet(defaultWallet, pda);
  console.log(r6);

  // const [tokenPda, r] = await createTokenMintAccount(defaultWallet);
  // console.log(tokenPda.toString(), r);
})();
