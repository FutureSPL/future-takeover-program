import * as anchor from "@coral-xyz/anchor";
import { Program, BN, Address } from "@coral-xyz/anchor";
import { FutureTakeoverProgram, IDL } from "../target/types/future_takeover_program";
import { Randomness, OracleJob } from "@switchboard-xyz/on-demand"; 
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  SYSVAR_INSTRUCTIONS_PUBKEY,
} from "@solana/web3.js";
import {
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountIdempotentInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

describe("future-takeover-program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;
  const wallet = provider.wallet as NodeWallet;
  const program = new Program(IDL, "GEvvRk67iniRpyYgeptxJdSHm3JBiQB757WtUrRm4GAd" as Address, provider);

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

  const [admin, user, oldMint, newMint] = Array.from({ length: 4 }, () =>
    Keypair.generate()
  );

  const [adminOldMintToken, adminNewMintToken, userOldMintToken, userNewMintToken] = [admin, user]
    .map((a) =>
      [oldMint, newMint].map((m) =>
        getAssociatedTokenAddressSync(m.publicKey, a.publicKey, false, tokenProgram)
      )
    )
    .flat();

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

  const takeoverNewMintVault = getAssociatedTokenAddressSync(newMint.publicKey, takeover, true, tokenProgram);
  const takeoverOldMintVault = getAssociatedTokenAddressSync(oldMint.publicKey, takeover, true, tokenProgram);

  const metadata = PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), metaplexTokenProgram.toBuffer(), newMint.publicKey.toBuffer()],
    metaplexTokenProgram
  )[0];

  it("Airdrop and create mints", async () => {
    let lamports = await getMinimumBalanceForRentExemptMint(connection);
    let tx = new Transaction();
    tx.instructions = [
      ...[admin, user].map((account) =>
        SystemProgram.transfer({
          fromPubkey: provider.publicKey,
          toPubkey: account.publicKey,
          lamports: 10 * LAMPORTS_PER_SOL,
        })
      ),
      ...[oldMint].map((mint) =>
        SystemProgram.createAccount({
          fromPubkey: provider.publicKey,
          newAccountPubkey: mint.publicKey,
          lamports,
          space: MINT_SIZE,
          programId: tokenProgram,
        })
      ),
      ...[
        { mint: oldMint.publicKey, authority: user.publicKey, ata: userOldMintToken },
      ]
      .flatMap((x) => [
        createInitializeMint2Instruction(x.mint, 9, x.authority, null, tokenProgram),
        createAssociatedTokenAccountIdempotentInstruction(provider.publicKey, x.ata, x.authority, x.mint, tokenProgram),
        createMintToInstruction(x.mint, x.ata, x.authority, 420_000_000 * 1e9, undefined, tokenProgram),
      ])
    ];

    await provider.sendAndConfirm(tx, [ oldMint, user ], {skipPreflight: true}).then(log);
  });

  // Setup Instructions
  // - Admin Setup
  it("Creates a New Admin Profile", async () => {
    await program.methods
      .adminInit("LEO")
      .accounts({
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
      .accounts({
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
      .accounts({
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
  xit("Creates a New Takeover", async () => {
    const takeoverArgs = {
      name: "Future",
      symbol: "FUT",
      uri: "uri",
      start: new BN(currentTimestamp + 25 * 60 * 60),
      end: new BN(currentTimestamp + 26 * 60 * 60),
      takeoverWallet: Keypair.generate().publicKey,
      presalePrice: new BN(1e5),
      fdmc: 0,
      presaleInflation: 100,
      treasuryInflation: 100,
      presaleInfation: 100,
      referral: null,
      referralSplit: null,
    }

    await program.methods
      .createTakeover(takeoverArgs)
      .accounts({
        admin: admin.publicKey,
        adminProfile,
        oldMint: oldMint.publicKey,
        newMint: newMint.publicKey,
        metadata,
        takeover,
        takeoverNewMintVault,
        systemProgram,
        sysvarInstructionProgram,
        metaplexTokenProgram,
        associatedTokenProgram
      })
      .signers([admin, newMint])
      .rpc({skipPreflight: true}).then(log).then(confirm);
  });

  xit("Update Takeover", async () => {
    const updateTakeoverArgs = {
      start: new BN(currentTimestamp + 25 * 60 * 60),
      end: new BN(currentTimestamp + 26 * 60 * 60),
      takeoverWallet: Keypair.generate().publicKey,
      presalePrice: new BN(1e5),
    }

    await program.methods
      .updateTakeover(updateTakeoverArgs)
      .accounts({
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
      .accounts({
        admin: admin.publicKey,
        adminProfile,
        newMint: newMint.publicKey,
        takeover,
        takeoverNewMintVault,
        systemProgram,
        tokenProgram,
        associatedTokenProgram,
      })
      .signers([admin])
      .rpc().then(log).then(confirm);
  });

  it("Creates a New Takeover", async () => {
      const takeoverArgs = {
        name: "Future",
        symbol: "FUT",
        uri: "uri",
        start: new BN(currentTimestamp),
        end: new BN(currentTimestamp + 60),
        takeoverWallet,
        presalePrice: new BN(1),
        fdmc: 0,
        presaleInflation: 100,
        treasuryInflation: 100,
        presaleInfation: 100,
        referral: Keypair.generate().publicKey,
        referralSplit: 500,
      }

      let tx = new Transaction

      tx.add( await program.methods
        .createTakeover(takeoverArgs)
        .accounts({
          admin: admin.publicKey,
          adminProfile,
          oldMint: oldMint.publicKey,
          newMint: newMint.publicKey,
          metadata,
          takeover,
          takeoverNewMintVault,
          systemProgram,
          sysvarInstructionProgram,
          metaplexTokenProgram,
          associatedTokenProgram
        })
        .instruction()
      );

      await provider.sendAndConfirm(tx, [admin, newMint], {skipPreflight: true}).then(confirm).then(log);
  });

  it("Swap Old Token", async () => {
    try {
      await program.methods
        .swapOldToken()
        .accounts({
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
        .buyPresale(new BN(200))
        .accounts({
          user: user.publicKey,
          takeover,
          presaleReceipt,
          takeoverVault,
          newMint: newMint.publicKey,
          oldMint: oldMint.publicKey,
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
        .accounts({
          admin: admin.publicKey,
          adminProfile,
          takeover,
          oldMint: oldMint.publicKey,
          newMint: newMint.publicKey,
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
        .accounts({
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

  it("Buy Presale - Success", async () => {
    try {
      await program.methods
        .buyPresale(new BN(75_000_000))
        .accounts({
          user: user.publicKey,
          takeover,
          presaleReceipt,
          takeoverVault,
          newMint: newMint.publicKey,
          oldMint: oldMint.publicKey,
          systemProgram,
        })
        .signers([user])
        .rpc({skipPreflight: true}).then(log).then(confirm);
    } catch (error) {
      console.log(error);
    }
  });

  it("Finalize Takeover - Successful", async () => {
    try {
      await program.methods
        .finalizeTakeover()
        .accounts({
          admin: admin.publicKey,
          adminProfile,
          takeover,
          oldMint: oldMint.publicKey,
          newMint: newMint.publicKey,
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

  it("Cleanup", async () => {
    await program.methods
      .cleanup()
      .accounts({
        admin: wallet.publicKey,
        adminProfile,
        newMint: newMint.publicKey,
        takeoverWallet,
        rewardWallet: Keypair.generate().publicKey,
        referralWallet: null,
        newMintRewardWalletToken,
        newMintReferralWalletToken: null,
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


});
