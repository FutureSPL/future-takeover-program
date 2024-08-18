import * as anchor from "@coral-xyz/anchor";
import { FutureTakeoverProgram } from "../target/types/future_takeover_program";

import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  SYSVAR_INSTRUCTIONS_PUBKEY,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";

import {
  TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  createMint,
  mintTo,
  getOrCreateAssociatedTokenAccount,
  getAssociatedTokenAddressSync,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("future-takeover-program", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const wallet = anchor.Wallet.local();
  const connection = anchor.getProvider().connection;
  const program = anchor.workspace.FutureTakeoverProgram as anchor.Program<FutureTakeoverProgram>;

  const tokenProgram1 = TOKEN_PROGRAM_ID;
  const tokenProgram = TOKEN_2022_PROGRAM_ID;
  const metaplexTokenProgram = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
  const systemProgram = SystemProgram.programId;
  const associatedTokenProgram = ASSOCIATED_TOKEN_PROGRAM_ID;
  const sysvarInstructionProgram = SYSVAR_INSTRUCTIONS_PUBKEY;

  function wait(ms: number) {
    return new Promise( resolve => setTimeout(resolve, ms) );
  }

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  // Accounts
  const takeoverWallet = Keypair.generate().publicKey;

  const [admin, user, oldMint, newMint] = Array.from({ length: 4 }, () =>
    Keypair.generate()
  );

  const adminProfile = PublicKey.findProgramAddressSync(
    [Buffer.from("admin"), admin.publicKey.toBuffer()],
    program.programId
  )[0];

  const takeover = PublicKey.findProgramAddressSync(
    [Buffer.from("takeover"), oldMint.publicKey.toBuffer()],
    program.programId
  )[0];

  const takeoverVault = PublicKey.findProgramAddressSync(
    [Buffer.from("takeover_vault"), takeover.toBuffer()],
    program.programId
  )[0];

  const swapReceipt = PublicKey.findProgramAddressSync(
    [Buffer.from("swap_receipt"), takeover.toBuffer(), user.publicKey.toBuffer()],
    program.programId
  )[0];

  const presaleReceipt = PublicKey.findProgramAddressSync(
    [Buffer.from("presale_receipt"), takeover.toBuffer(), user.publicKey.toBuffer()],
    program.programId
  )[0];

  const userOldMintToken = getAssociatedTokenAddressSync(oldMint.publicKey, user.publicKey, false, tokenProgram1);
  const takeoverOldMintVault = getAssociatedTokenAddressSync(oldMint.publicKey, takeover, true, tokenProgram1);

  const userNewMintToken = getAssociatedTokenAddressSync(newMint.publicKey, user.publicKey, false, tokenProgram);
  const takeoverNewMintVault = getAssociatedTokenAddressSync(newMint.publicKey, takeover, true, tokenProgram);

  const metadata = PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), metaplexTokenProgram.toBuffer(), newMint.publicKey.toBuffer()],
    metaplexTokenProgram
  )[0];

  it("Airdrop and create mints", async () => {
    await connection.requestAirdrop(user.publicKey, 10 * LAMPORTS_PER_SOL);
    await connection.requestAirdrop(admin.publicKey, 10 * LAMPORTS_PER_SOL);
    await createMint(connection, wallet.payer, wallet.publicKey, null, 9, oldMint, null, tokenProgram1);
    await getOrCreateAssociatedTokenAccount(connection, wallet.payer, oldMint.publicKey, user.publicKey, null, null, null, tokenProgram1);
    await mintTo(connection, wallet.payer, oldMint.publicKey, userOldMintToken, wallet.payer, 420_000_000 * 1e9, [], null, tokenProgram1);
  });

  // Setup Instructions

  // - Admin Setup
  it("Creates a New Admin Profile", async () => {
    await program.methods
      .adminInit("LEO")
      .accountsPartial({
        owner: wallet.publicKey,
        newAdmin: admin.publicKey,
        adminProfile,
        systemProgram: SystemProgram.programId,
      })
      .signers([wallet.payer])
      .rpc().then(log).then(confirm);
  });

  xit("Cancel an Admin Profile", async () => {
    await program.methods
      .adminDelete()
      .accountsPartial({
        owner: wallet.publicKey,
        oldAdmin: admin.publicKey,
        adminProfile,
        systemProgram: SystemProgram.programId,
      })
      .signers([wallet.payer])
      .rpc().then(log).then(confirm);
  });

  xit("Creates a New Admin Profile", async () => {
    await program.methods
      .adminInit("LEO")
      .accountsPartial({
        owner: wallet.publicKey,
        newAdmin: admin.publicKey,
        adminProfile,
        systemProgram: SystemProgram.programId,
      })
      .signers([wallet.payer])
      .rpc().then(log).then(confirm);
  });

  const currentTimestamp = Math.floor(Date.now() / 1000);

  // - Takeover Setup
  xit("Creates a New Takeover - Normal Token", async () => {
    const createTakeoverArgs = {
      name: "Future", // string
      symbol: "FUT", // string
      uri: "https://arweave.net/123", // string
      start: new anchor.BN(currentTimestamp + 25 * 60 * 60), // i64
      end: new anchor.BN(currentTimestamp + 26 * 60 * 60), // i64
      takeoverWallet: takeoverWallet, // pubkey
      presalePrice: new anchor.BN(1e5), // u64
      fdmc: 0, // u8
      presaleInflation: 100, // u16
      treasuryInflation: 100, // u16
      rewardsInflation: 100, // u16
      referral: null, // option<pubkey>
      referralSplit: null, // option<u16>
      tokenExtension: null, // option<TokenExtensionArgs>
    };

    await program.methods
      .createTakeover(createTakeoverArgs)
      .accountsPartial({
        admin: admin.publicKey,
        adminProfile,
        takeover,
        oldMint: oldMint.publicKey,
        newMint: newMint.publicKey,
        metadata,
        takeoverNewMintVault,
        systemProgram,
        sysvarInstructionProgram,
        metaplexTokenProgram,
        tokenProgram,
        rent: SYSVAR_RENT_PUBKEY,
        associatedTokenProgram
      })
      .signers([admin, newMint])
      .rpc({skipPreflight: false}).then(log).then(confirm);
  });

  xit("Update Takeover", async () => {
    const updateTakeoverArgs = {
      start: new anchor.BN(currentTimestamp + 25 * 60 * 60),
      end: new anchor.BN(currentTimestamp + 26 * 60 * 60),
      takeoverWallet: Keypair.generate().publicKey,
      presalePrice: new anchor.BN(1e5),
    }

    await program.methods
      .updateTakeover(updateTakeoverArgs)
      .accountsPartial({
        admin: admin.publicKey,
        adminProfile,
        takeover,
        systemProgram,
      })
      .signers([admin])
      .rpc().then(log).then(confirm);
  });

  xit("Cancel Takeover", async () => {
    await program.methods
      .cancelTakeover()
      .accountsPartial({
        admin: admin.publicKey,
        adminProfile,
        takeover,
        newMint: newMint.publicKey,
        takeoverNewMintVault,
        systemProgram,
        tokenProgram,
        associatedTokenProgram,
      })
      .signers([admin])
      .rpc().then(log).then(confirm);
  });

  it("Creates a New Takeover - Token2022", async () => {
    const transferFeeArgs = {
      feeAuthority: wallet.publicKey,
      transferFeeBasisPoints: 100,
      maximumFee: new anchor.BN(1e5),
    };
    
    const interestBearingArgs = {
      rateAuthority: wallet.publicKey,
      rate: 100,
    };
    
    const permanentDelegateArgs = {
      delegateAuthority: wallet.publicKey,
    };
    
    const closeMintArgs = {
      closeMintAuthority: wallet.publicKey, 
    };

    const tokenExtensionArgs = {
      transferFee: transferFeeArgs,
      interestBearing: interestBearingArgs,
      permanentDelegate: permanentDelegateArgs,
      closeMint: closeMintArgs, 
      transferHook: null
    };
    
    
    const createTakeoverArgs = {
      name: "Future",
      symbol: "FUT",
      uri: "https://arweave.net/123",
      start: new anchor.BN(currentTimestamp + 25 * 60 * 60),
      end: new anchor.BN(currentTimestamp + 26 * 60 * 60),
      takeoverWallet,
      presalePrice: new anchor.BN(1e5),
      fdmc: 0,
      presaleInflation: 100,
      treasuryInflation: 100,
      rewardsInflation: 100,
      referral: null,
      referralSplit: null,
      tokenExtension: tokenExtensionArgs,
    };

    await program.methods
      .createTakeover(createTakeoverArgs)
      .accountsPartial({
        admin: admin.publicKey,
        adminProfile,
        takeover,
        oldMint: oldMint.publicKey,
        newMint: newMint.publicKey,
        metadata,
        takeoverNewMintVault,
        systemProgram,
        sysvarInstructionProgram,
        metaplexTokenProgram,
        tokenProgram,
        rent: SYSVAR_RENT_PUBKEY,
        associatedTokenProgram
      })
      .signers([admin, newMint])
      .rpc({skipPreflight: false});
  });

  xit("Swap Old Token", async () => {
    try {
      await program.methods
        .swapOldToken()
        .accountsPartial({
          user: user.publicKey,
          takeover,
          swapReceipt,
          oldMint: oldMint.publicKey,
          userOldMintToken,
          takeoverOldMintVault,
          systemProgram,
          tokenProgram,
          associatedTokenProgram,
        })
        .signers([user])
        .rpc({skipPreflight: true}).then(log).then(confirm);
    } catch (error) {
      console.log(error);
    }
  });

  xit("Buy Presale - Fail", async () => {
    try {
      await program.methods
        .buyPresale(new anchor.BN(200))
        .accountsPartial({
          user: user.publicKey,
          takeover,
          presaleReceipt,
          takeoverVault,
          newMint: newMint.publicKey,
          systemProgram,
        })
        .signers([user])
        .rpc({skipPreflight: true}).then(log).then(confirm);
    } catch (error) {
      console.log(error);
    }
  });

  xit("Finalize Takeover - Failed", async () => {
    try {
      await program.methods
        .finalizeTakeover()
        .accountsPartial({
          admin: admin.publicKey,
          adminProfile,
          takeover,
          systemProgram,
        })
        .signers([admin])
        .rpc({skipPreflight: true}).then(log).then(confirm);
    } catch (error) {
      console.log(error);
    }
  });

  xit("Claim Refund", async () => {
    try {
      await program.methods
        .claimRefund()
        .accountsPartial({
          user: user.publicKey,
          takeover,
          presaleReceipt,
          swapReceipt,
          newMint: newMint.publicKey,
          oldMint: oldMint.publicKey,
          takeoverVault,
          takeoverOldMintVault,
          userOldMintToken,
          systemProgram,
          tokenProgram,
          associatedTokenProgram,
        })
        .signers([user])
        .rpc({skipPreflight: true}).then(log).then(confirm);
    } catch (error) {
      console.log(error);
    }
  });

  xit("Buy Presale - Success", async () => {
    try {
      await program.methods
        .buyPresale(new anchor.BN(75_000_000))
        .accountsPartial({
          user: user.publicKey,
          takeover,
          presaleReceipt,
          takeoverVault,
          newMint: newMint.publicKey,
          systemProgram,
        })
        .signers([user])
        .rpc({skipPreflight: true}).then(log).then(confirm);
    } catch (error) {
      console.log(error);
    }
  });

  xit("Finalize Takeover - Successful", async () => {
    try {
      await program.methods
        .finalizeTakeover()
        .accountsPartial({
          admin: admin.publicKey,
          adminProfile,
          takeover,
          systemProgram,
        })
        .signers([admin])
        .rpc({skipPreflight: true}).then(log).then(confirm);
    } catch (error) {
      console.log(error);
    }
  });

  const rewardWallet = Keypair.generate().publicKey;
  const newMintTakeoverWalletToken = getAssociatedTokenAddressSync(newMint.publicKey, takeoverWallet, false, tokenProgram);
  const newMintRewardWalletToken = getAssociatedTokenAddressSync(newMint.publicKey, rewardWallet, false, tokenProgram);
  const newMintTakeoverVault = getAssociatedTokenAddressSync(newMint.publicKey, takeover, true, tokenProgram);

  xit ("Distrubute Rewards", async () => {
    await program.methods
      .distributeRewards()
      .accountsPartial({
        admin: admin.publicKey,
        adminProfile,
        newMint: newMint.publicKey,
        rewardWallet,
        referralWallet: null,
        newMintRewardWalletToken,
        newMintReferralWalletToken: null,
        takeover,
        newMintTakeoverVault,
        systemProgram,
        tokenProgram,
        associatedTokenProgram,
      })
      .signers([admin])
      .rpc({skipPreflight: true}).then(log).then(confirm);
  });

  xit("Cleanup", async () => {
    await program.methods
      .cleanup()
      .accountsPartial({
        admin: wallet.publicKey,
        adminProfile,
        newMint: newMint.publicKey,
        takeoverWallet,
        newMintTakeoverWalletToken,
        takeover,
        newMintTakeoverVault,
        takeoverVault,
        systemProgram,
        tokenProgram,
        associatedTokenProgram,
      })
      .signers([wallet.payer])
      .rpc({skipPreflight: true}).then(log).then(confirm);
  });

  xit("Claim Tokens", async () => {
    await program.methods
      .claimTokens()
      .accountsPartial({
        user: user.publicKey,
        takeover,
        presaleReceipt,
        swapReceipt,
        newMint: newMint.publicKey,
        takeoverNewMintVault,
        userNewMintToken,
        systemProgram,
        tokenProgram,
        associatedTokenProgram,
      })
      .signers([user])
      .rpc({skipPreflight: true}).then(log).then(confirm);
  });

  xit("Claim Remaining Tokens", async () => {
    await program.methods
      .claimRemainingTokens()
      .accountsPartial({
        admin: admin.publicKey,
        adminProfile,
        newMint: newMint.publicKey,
        takeoverWallet,
        newMintTakeoverWalletToken,
        takeover,
        newMintTakeoverVault,
        systemProgram,
        tokenProgram,
        associatedTokenProgram,
      })
      .signers([user])
      .rpc({skipPreflight: true}).then(log).then(confirm);
  });
});
