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
  burn,
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
  // const tokenProgram = TOKEN_2022_PROGRAM_ID;
  const tokenProgram = TOKEN_PROGRAM_ID;
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

  const [admin, user, oldMint, oldMint2, oldMint3, newMint] = Array.from({ length: 6 }, () =>
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

  const userOldMin2tToken = getAssociatedTokenAddressSync(oldMint2.publicKey, user.publicKey, false, tokenProgram1);
  const takeoverOldMint2Vault = getAssociatedTokenAddressSync(oldMint2.publicKey, takeover, true, tokenProgram1);

  const userOldMin3tToken = getAssociatedTokenAddressSync(oldMint3.publicKey, user.publicKey, false, tokenProgram1);
  const takeoverOldMint3Vault = getAssociatedTokenAddressSync(oldMint3.publicKey, takeover, true, tokenProgram1);

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
      oldMints: null,
      decimals: null,
      referral: null, 
      referralSplit: null,
      tokenExtension: null,
    };

    await program.methods
      .createTakeover(createTakeoverArgs)
      .accountsPartial({
        admin: admin.publicKey,
        adminProfile,
        takeover,
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
      .remainingAccounts([
        { isSigner: false, isWritable: false, pubkey: oldMint.publicKey }
      ])
      .signers([admin, newMint])
      .rpc({skipPreflight: true}).then(log).then(confirm);

    console.log("Presale Amount: ", (await program.account.takeover.fetch(takeover)).inflationAmount.presaleAmount.toString());
    console.log("Treasury Amount: ", (await program.account.takeover.fetch(takeover)).inflationAmount.treasuryAmount.toString());
    console.log("Rewards Amount: ", (await program.account.takeover.fetch(takeover)).inflationAmount.rewardsAmount.toString());
    console.log("OldMint 1: ", (await program.account.takeover.fetch(takeover)).oldMints.oldMint.toString());
    console.log("OldMint 2: ", (await program.account.takeover.fetch(takeover)).oldMints.oldMint2.toString());
  });

  xit("Creates a New Takeover - Token2022", async () => {
    const transferFeeArgs = {
      feeAuthority: wallet.publicKey,
      transferFeeBasisPoints: 100,
    };
    
    const interestBearingArgs = {
      rateAuthority: wallet.publicKey,
      rate: 100,
    };
    
    const closeMintArgs = {
      closeMintAuthority: wallet.publicKey, 
    };

    const tokenExtensionArgs = {
      transferFee: transferFeeArgs,
      interestBearing: interestBearingArgs,
      permanentDelegate: null,
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
      oldMints: null,
      decimals: null,
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
      .remainingAccounts([
        { isSigner: false, isWritable: false, pubkey: oldMint.publicKey }
      ])
      .signers([admin, newMint])
      .rpc({skipPreflight: false});

      console.log("Presale Amount: ", (await program.account.takeover.fetch(takeover)).inflationAmount.presaleAmount.toString());
      console.log("Treasury Amount: ", (await program.account.takeover.fetch(takeover)).inflationAmount.treasuryAmount.toString());
      console.log("Rewards Amount: ", (await program.account.takeover.fetch(takeover)).inflationAmount.rewardsAmount.toString());
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
      })
      .signers([admin])
      .rpc({skipPreflight: true}).then(log).then(confirm);
  });

  xit("Creates a New Multiple Token Takeover - Normal Token", async () => {
    await createMint(connection, wallet.payer, wallet.publicKey, null, 9, oldMint2, null, tokenProgram1);
    await getOrCreateAssociatedTokenAccount(connection, wallet.payer, oldMint2.publicKey, user.publicKey, null, null, null, tokenProgram1);
    await mintTo(connection, wallet.payer, oldMint2.publicKey, userOldMin2tToken, wallet.payer, 420_000_000 * 1e9, [], null, tokenProgram1);

    await createMint(connection, wallet.payer, wallet.publicKey, null, 9, oldMint3, null, tokenProgram1);
    await getOrCreateAssociatedTokenAccount(connection, wallet.payer, oldMint3.publicKey, user.publicKey, null, null, null, tokenProgram1);
    await mintTo(connection, wallet.payer, oldMint3.publicKey, userOldMin3tToken, wallet.payer, 420_000_000 * 1e9, [], null, tokenProgram1);

    const transferFeeArgs = {
      feeAuthority: wallet.publicKey,
      transferFeeBasisPoints: 100,
    };
    
    const interestBearingArgs = {
      rateAuthority: wallet.publicKey,
      rate: 100,
    };
    
    const closeMintArgs = {
      closeMintAuthority: wallet.publicKey, 
    };

    const tokenExtensionArgs = {
      transferFee: transferFeeArgs,
      interestBearing: interestBearingArgs,
      permanentDelegate: null,
      closeMint: closeMintArgs, 
      transferHook: null
    };

    const oldMints = {
      oldMint: oldMint.publicKey,
      weightPercentage: 30,
      oldMint2: oldMint2.publicKey,
      weightPercentage2: 50,
      oldMint3: oldMint3.publicKey,
      weightPercentage3: 20,
    }

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
      oldMints,
      decimals: 9,
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
      .remainingAccounts([
        { isSigner: false, isWritable: false, pubkey: oldMint.publicKey },
        { isSigner: false, isWritable: false, pubkey: oldMint2.publicKey },
        { isSigner: false, isWritable: false, pubkey: oldMint3.publicKey },
      ])
      .signers([admin, newMint])
      .rpc({skipPreflight: true}).then(log).then(confirm);

      console.log("Presale Amount: ", (await program.account.takeover.fetch(takeover)).inflationAmount.presaleAmount.toString());
      console.log("Treasury Amount: ", (await program.account.takeover.fetch(takeover)).inflationAmount.treasuryAmount.toString());
      console.log("Rewards Amount: ", (await program.account.takeover.fetch(takeover)).inflationAmount.rewardsAmount.toString());
  });

  it("Creates a New Multiple Token Takeover - Token2022", async () => {
    await createMint(connection, wallet.payer, wallet.publicKey, null, 9, oldMint2, null, tokenProgram1);
    await getOrCreateAssociatedTokenAccount(connection, wallet.payer, oldMint2.publicKey, user.publicKey, null, null, null, tokenProgram1);
    await mintTo(connection, wallet.payer, oldMint2.publicKey, userOldMin2tToken, wallet.payer, 420_000_000 * 1e9, [], null, tokenProgram1);

    await createMint(connection, wallet.payer, wallet.publicKey, null, 9, oldMint3, null, tokenProgram1);
    await getOrCreateAssociatedTokenAccount(connection, wallet.payer, oldMint3.publicKey, user.publicKey, null, null, null, tokenProgram1);
    await mintTo(connection, wallet.payer, oldMint3.publicKey, userOldMin3tToken, wallet.payer, 420_000_000 * 1e9, [], null, tokenProgram1);

    const oldMints = {
      oldMint: oldMint.publicKey,
      weightPercentage: 30,
      oldMint2: oldMint2.publicKey,
      weightPercentage2: 50,
      oldMint3: oldMint3.publicKey,
      weightPercentage3: 20,
    }

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
      oldMints,
      decimals: 9,
      referral: null, 
      referralSplit: null,
      tokenExtension: null,
    };

    await program.methods
      .createTakeover(createTakeoverArgs)
      .accountsPartial({
        admin: admin.publicKey,
        adminProfile,
        takeover,
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
      .remainingAccounts([
        { isSigner: false, isWritable: false, pubkey: oldMint.publicKey },
        { isSigner: false, isWritable: false, pubkey: oldMint2.publicKey },
        { isSigner: false, isWritable: false, pubkey: oldMint3.publicKey },
      ])
      .signers([admin, newMint])
      .rpc({skipPreflight: false}).then(log).then(confirm);

      console.log("Presale Amount: ", (await program.account.takeover.fetch(takeover)).inflationAmount.presaleAmount.toString());
      console.log("Treasury Amount: ", (await program.account.takeover.fetch(takeover)).inflationAmount.treasuryAmount.toString());
      console.log("Rewards Amount: ", (await program.account.takeover.fetch(takeover)).inflationAmount.rewardsAmount.toString());
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
      })
      .signers([admin])
      .rpc({skipPreflight: true}).then(log).then(confirm);
  });

  // Note: here the TokenProgram is based on the old_mint token program
  it("Swap Old Token", async () => {
    console.log("User Token Balance: ", (await connection.getTokenAccountBalance(userOldMintToken)).value.amount.toString());
    console.log("User Token2 Balance: ", (await connection.getTokenAccountBalance(userOldMin2tToken)).value.amount.toString());
    console.log("User Token3 Balance: ", (await connection.getTokenAccountBalance(userOldMin3tToken)).value.amount.toString());

    await program.methods
      .swapOldToken()
      .accountsPartial({
        user: user.publicKey,
        takeover,
        swapReceipt,
        systemProgram,
        tokenProgram: tokenProgram1,
        associatedTokenProgram,
      })
      .remainingAccounts([
        { isSigner: false, isWritable: false, pubkey: oldMint.publicKey },
        { isSigner: false, isWritable: true, pubkey: userOldMintToken },
        { isSigner: false, isWritable: true, pubkey: takeoverOldMintVault },
        { isSigner: false, isWritable: false, pubkey: oldMint2.publicKey },
        { isSigner: false, isWritable: true, pubkey: userOldMin2tToken },
        { isSigner: false, isWritable: true, pubkey: takeoverOldMint2Vault },
        { isSigner: false, isWritable: false, pubkey: oldMint3.publicKey },
        { isSigner: false, isWritable: true, pubkey: userOldMin3tToken },
        { isSigner: false, isWritable: true, pubkey: takeoverOldMint3Vault },
      ])
      .signers([user])
      .rpc({skipPreflight: true}).then(log).then(confirm);

      console.log("Swapped Amount from Swap Receipt: ", (await program.account.swapReceipt.fetch(swapReceipt)).swappedAmount.toString());
      console.log("Swapped Amount from Takeover: ", (await program.account.takeover.fetch(takeover)).tokenSwapped.toString());

      console.log("Swapped Amount Token1: ", (await program.account.swapReceipt.fetch(swapReceipt)).swappedOldMints.amount.toString());
      console.log("Swapped Amount Token2: ", (await program.account.swapReceipt.fetch(swapReceipt)).swappedOldMints.amount2.toString());
      console.log("Swapped Amount Token3: ", (await program.account.swapReceipt.fetch(swapReceipt)).swappedOldMints.amount3.toString());
  });

  it("Buy Presale", async () => {
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

      console.log("Presale Amount: ", (await program.account.presaleReceipt.fetch(presaleReceipt)).presaleAmount.toString());
      console.log("Presale Amount: ", (await program.account.takeover.fetch(takeover)).presaleClaimed.toString());
  });

  it("Buy Presale", async () => {
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

      console.log("Presale Amount: ", (await program.account.presaleReceipt.fetch(presaleReceipt)).presaleAmount.toString());
      console.log("Presale Amount: ", (await program.account.takeover.fetch(takeover)).presaleClaimed.toString());
  });

  it("Finalize Takeover", async () => {
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
  });

  // Note: here the TokenProgram is based on the old_mint token program
  it("Claim Refund", async () => {
    await program.methods
      .claimRefund()
      .accountsPartial({
        user: user.publicKey,
        takeover,
        presaleReceipt,
        swapReceipt,
        newMint: newMint.publicKey,
        takeoverVault,
        systemProgram,
        tokenProgram: tokenProgram1,
        associatedTokenProgram,
      })
      .remainingAccounts([
        { isSigner: false, isWritable: false, pubkey: oldMint.publicKey },
        { isSigner: false, isWritable: true, pubkey: userOldMintToken },
        { isSigner: false, isWritable: true, pubkey: takeoverOldMintVault },
        { isSigner: false, isWritable: false, pubkey: oldMint2.publicKey },
        { isSigner: false, isWritable: true, pubkey: userOldMin2tToken },
        { isSigner: false, isWritable: true, pubkey: takeoverOldMint2Vault },
        { isSigner: false, isWritable: false, pubkey: oldMint3.publicKey },
        { isSigner: false, isWritable: true, pubkey: userOldMin3tToken },
        { isSigner: false, isWritable: true, pubkey: takeoverOldMint3Vault },
      ])
      .signers([user])
      .rpc({skipPreflight: true}).then(log).then(confirm);

      console.log("Presale Amount: ", (await program.account.presaleReceipt.fetch(presaleReceipt)).presaleAmount.toString());

      console.log("Swapped Amount: ", (await program.account.swapReceipt.fetch(swapReceipt)).swappedAmount.toString());
      console.log("Swapped Amount Token1: ", (await program.account.swapReceipt.fetch(swapReceipt)).swappedOldMints.amount.toString());
      console.log("Swapped Amount Token2: ", (await program.account.swapReceipt.fetch(swapReceipt)).swappedOldMints.amount2.toString());
      console.log("Swapped Amount Token3: ", (await program.account.swapReceipt.fetch(swapReceipt)).swappedOldMints.amount3.toString());

      console.log("User Token Balance: ", (await connection.getTokenAccountBalance(userOldMintToken)).value.amount.toString());
      console.log("User Token2 Balance: ", (await connection.getTokenAccountBalance(userOldMin2tToken)).value.amount.toString());
      console.log("User Token3 Balance: ", (await connection.getTokenAccountBalance(userOldMin3tToken)).value.amount.toString());

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
      .rpc({skipPreflight: false}).then(log).then(confirm);
  });

  xit("Cleanup", async () => {
    await program.methods
      .cleanup()
      .accountsPartial({
        admin: admin.publicKey,
        adminProfile,
        takeover,
        newMintTakeoverVault,
        takeoverVault,
        newMint: newMint.publicKey,
        takeoverWallet,
        newMintTakeoverWalletToken,
        systemProgram,
        tokenProgram,
        associatedTokenProgram,
      })
      .signers([admin])
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
      .rpc({skipPreflight: false}).then(log).then(confirm);
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
      .signers([admin])
      .rpc({skipPreflight: false}).then(log).then(confirm);
  });
});
