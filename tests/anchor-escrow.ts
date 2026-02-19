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
  createNativeMintInstructionData,
  TokenAccountNotFoundError
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

    makerAtaA = await createAssociatedTokenAccount(
      connection,
      maker,
      mintA,
      maker.publicKey,
      confirmOpts
    )

    takerAtaB = await createAssociatedTokenAccount(
      connection,
      taker,
      mintB,
      taker.publicKey,
      confirmOpts
    )

    await mintTo(
      connection,
      maker,
      mintA,
      makerAtaA,
      maker.publicKey,
      100_000_000,
      undefined,
      confirmOpts,
    );

    await mintTo(
      connection,
      taker,
      mintB,
      takerAtaB,
      taker.publicKey,
      100_000_000,
      undefined,
      confirmOpts
    );

  });

  it("make", async() => {
    const escrow = getEscrowPda(maker.publicKey, seed);
    const vault = getVaultAta(escrow, mintA);

  await program.methods
    .make(seed, receiveAmount, depositAmount)
    .accountsPartial({
      maker: maker.publicKey,
      mintA,
      mintB,
      makerAtaA,
      escrow,
      vault,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([maker])
    .rpc(confirmOpts);

    const escrowAccount = await program.account.escrow.fetch(escrow);
    expect(escrowAccount.seeds.toNumber()).to.equal(seed.toNumber());
    expect(escrowAccount.maker.toBase58()).to.equal(maker.publicKey.toBase58());
    expect(escrowAccount.mintA.toBase58()).to.equal(mintA.toBase58());
    expect(escrowAccount.mintB.toBase58()).to.equal(mintB.toBase58());
    expect(escrowAccount.receive.toNumber()).to.equal(receiveAmount.toNumber());

    const vaultAccount = await getAccount(connection, vault);
    expect(Number(vaultAccount.amount)).to.equal(depositAmount.toNumber());
  });

  it("take", async() => {
    const escrow = getEscrowPda(maker.publicKey, seed);
    const vault = getVaultAta(escrow, mintA);
    const takerAtaA = getAssociatedTokenAddressSync(mintA, taker.publicKey);
    const makerAtaB = getAssociatedTokenAddressSync(mintB, maker.publicKey);

    const takerAtaBBefore= await getAccount(connection, takerAtaB);

    await program.methods
    .take(receiveAmount)
    .accountsPartial({
      taker: taker.publicKey,
      maker: maker.publicKey,
      mintA,
      mintB,
      takerAtaA,
      takerAtaB,
      makerAtaB,
      escrow,
      vault,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([taker])
    .rpc(confirmOpts);


    // confirm that taker received the correct amount of mint A
    const takerAtaAAccount = await getAccount(connection, takerAtaA);
    expect(Number(takerAtaAAccount.amount)).to.equal(depositAmount.toNumber());

    // confirm that maker received the correct amount of mint B
    const makerAtaBAccount = await getAccount(connection, makerAtaB);
    expect(Number(makerAtaBAccount.amount)).to.equal(receiveAmount.toNumber());

    // confirm that taker's mint B balance decreased by the correct amount
    const takerAtaBAfter = await getAccount(connection, takerAtaB);
    expect(Number(takerAtaBAfter.amount)).to.equal(
      Number(takerAtaBBefore.amount) - receiveAmount.toNumber()
    );

    // confirm that the escrow is now closed
    const escrowAccount = await connection.getAccountInfo(escrow);
    expect(escrowAccount).to.be.null;

    // confirm that the vault account is now closed
    const vaultAccount = await connection.getAccountInfo(vault);
    expect(vaultAccount).to.be.null;

  });

  it("refund", async() => {});

});
