import * as anchor from "@coral-xyz/anchor";
import { Program, BN} from "@coral-xyz/anchor";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import {
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  getAssociatedTokenAddressSync,
  getAccount,
  TOKEN_PROGRAM_ID,
  createNativeMintInstructionData
} from "@solana/spl-token";

import { 
  Keypair,
  PublicKey,
  LAMPORTS_PER_SOL,
  ConfirmOptions,
  Connection,
} from "@solana/web3.js";

import { expect } from "chai";

const confirmOpts: ConfirmOptions = {commitment: "confirmed"};


describe("anchor-escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const connection = anchor.getProvider().connection;
  

  const program = anchor.workspace.anchorEscrow as Program<AnchorEscrow>;

  const maker = Keypair.generate();
  const taker = Keypair.generate();
  let mintA: PublicKey;
  let mintB: PublicKey;
  let makerAtaA: PublicKey;
  let takerAtaB: PublicKey;

  const seed = new BN(1);
  const depositAmount = new BN(10_000_000);
  const receiveAmount = new BN(5_000_000);
  const decimals = 6;

  async function airdrop(to: PublicKey, amount: number) {
    const latestBlockhash = await connection.getLatestBlockhash();
    const sig = await connection.requestAirdrop(to, amount);
    await connection.confirmTransaction(
      { signature: sig, ...latestBlockhash},
      "confirmed",
    );
    console.log(`Aidrop of ${amount} SOL to ${to}`);
  }

  function getEscrowPda(makerKey: PublicKey, escrowSeed: BN): PublicKey {
    return PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        makerKey.toBuffer(),
        escrowSeed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId,
    )[0];
  }

  function getVaultAta(escrow: PublicKey, mint: PublicKey): PublicKey {
    return getAssociatedTokenAddressSync(mint, escrow, true);
  }

  before(async () => {
    await airdrop(maker.publicKey, 10 * LAMPORTS_PER_SOL);
    await airdrop(taker.publicKey, 10 * LAMPORTS_PER_SOL);

    mintA = await createMint(
      connection,
      maker,
      maker.publicKey,
      null,
      decimals,
      undefined,
      confirmOpts
    );

    mintB = await createMint(
      connection,
      taker,
      taker.publicKey,
      null,
      decimals,
      undefined,
      confirmOpts
    );

  });

  it("make", async() => {});

  it("take", async() => {});

  it("refund", async() => {});

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
