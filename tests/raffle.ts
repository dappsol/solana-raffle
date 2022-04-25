import * as anchor from "@project-serum/anchor";
import { Program, Spl } from "@project-serum/anchor";
import { Raffle } from "../target/types/raffle";
import BN from "bn.js"

import fs from "fs";
// var jsonFile = "/home/kts/.config/solana/id.json";
var jsonFile = "/home/Guardian/dope-pirates-staking-contract-v3/client.json";
var parsed = JSON.parse(fs.readFileSync(jsonFile));

import {
  AccountLayout,
  TOKEN_PROGRAM_ID,
  createAccount,
  createMint,
  getMint,
  getOrCreateAssociatedTokenAccount,
  getAccount,
  mintTo,
  createInitializeAccountInstruction,
} from "@solana/spl-token";

import { ConfirmOptions } from "@solana/web3.js";
import { token } from "@project-serum/anchor/dist/cjs/utils";
const { 
  SystemProgram, 
  Keypair, 
  PublicKey, 
  LAMPORTS_PER_SOL, 
  clusterApiUrl, 
  SYSVAR_RENT_PUBKEY,
  SYSVAR_CLOCK_PUBKEY,
 } = anchor.web3;

describe("raffle", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Raffle as Program<Raffle>;

  it("Is initialized!", async () => {
    // Add your test here.
    const provider = anchor.AnchorProvider.env();
    const signer = Keypair.fromSecretKey(new Uint8Array(parsed));

    let bal = await provider.connection.getBalance(
      signer.publicKey,
      "confirmed"
    );
    console.log("bal = ", bal);
    console.log("wallet = ", provider.wallet.publicKey.toBase58());

    // nft mint
    const token_mint = await createMint(
      provider.connection,
      signer,
      signer.publicKey,
      signer.publicKey,
      0,
    );
    console.log("mintkey = ", token_mint.toBase58());

    const ownerTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      signer,
      token_mint,
      signer.publicKey,
    );
    console.log("ownerTokenAccount = ", ownerTokenAccount.address.toBase58());

    await mintTo(
      provider.connection,
      signer,
      token_mint,
      ownerTokenAccount.address,
      signer.publicKey,
      2,
    );

    // ticket mint
    // const ticket_mint = await createMint(
    //   provider.connection,
    //   signer,
    //   signer.publicKey,
    //   signer.publicKey,
    //   0,
    // );
    // console.log("mintkey = ", token_mint.toBase58());

    // const ownerTokenAccount = await getOrCreateAssociatedTokenAccount(
    //   provider.connection,
    //   signer,
    //   token_mint,
    //   signer.publicKey,
    // );
    // console.log("ownerTokenAccount = ", ownerTokenAccount.address.toBase58());

    // await mintTo(
    //   provider.connection,
    //   signer,
    //   token_mint,
    //   ownerTokenAccount.address,
    //   signer.publicKey,
    //   1,
    // );

    const mintInfo = await getMint(provider.connection, token_mint);
    console.log("mintInfo.supply = ", mintInfo.supply);

    let rent = await provider.connection.getAccountInfoAndContext(SYSVAR_RENT_PUBKEY, "confirmed");
    console.log("rent = ", rent.value.owner.toBase58(), SYSVAR_RENT_PUBKEY.toBase58());

    // let [vaultPDA, _nonce] = await PublicKey.findProgramAddress(
    //   [Buffer.from("vault")],
    //   program.programId
    // );

    // console.log("vaultPda = ", vaultPDA.toString(), "nonce", _nonce);
    // let sig1 = await program.rpc.createVault(_nonce, {
    //   accounts: {
    //     vault: vaultPDA,
    //     admin: signer.publicKey,
    //     systemProgram: SystemProgram.programId,
    //   },
    // });
    // await provider.connection.confirmTransaction(sig1);
    
    let [lotteryPDA, _lottery_nonce] = await PublicKey.findProgramAddress(
      [Buffer.from("lottery"), signer.publicKey.toBuffer(), token_mint.toBuffer()],
      program.programId,
    );
    console.log("lotteryPDA = ", lotteryPDA.toBase58(), "\nlottery_nonce = ", _lottery_nonce);

    // let lottery = Keypair.generate();

    let [escrowPDA, _escrow_nonce] = await PublicKey.findProgramAddress(
      [Buffer.from("lottery_escrow"), signer.publicKey.toBuffer(), lotteryPDA.toBuffer()],
      program.programId,
    );
    console.log("escrowPDA = ", escrowPDA.toBase58(), "\nescrow_nonce = ", _escrow_nonce);

    let [vaultPDA, _vault_nonce] = await PublicKey.findProgramAddress(
      [Buffer.from("lottery_escrow_vault"), escrowPDA.toBuffer()],
      program.programId,
    );
    console.log("vaultPDA = ", vaultPDA.toBase58(), "\nvault_nonce = ", _vault_nonce);

    let slot = await provider.connection.getSlot("finalized");
    console.log("slot = ", slot);
    const time = await provider.connection.getBlockTime(slot);
    console.log("time = ", time);
    let sig = await program.rpc.upload( new BN(time), new BN(time + 600), {
      accounts: {
        lottery: lotteryPDA,
        user: signer.publicKey,
        userNftAccount: ownerTokenAccount.address,
        nftMint: token_mint,
        escrow: escrowPDA,
        vault: vaultPDA,
        rentSysvar: SYSVAR_RENT_PUBKEY,
        clockSysvar: SYSVAR_CLOCK_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      },
    });
    let res = await provider.connection.confirmTransaction(sig);
    console.log("confirmed tx = ", sig);

    let ownerTokenAccountInfo = await getAccount(
      provider.connection,
      ownerTokenAccount.address,
    );
    console.log("ownerTokenAccountinfo.amount = ", ownerTokenAccountInfo.amount);

    bal = await provider.connection.getBalance(
      vaultPDA,
      "confirmed"
    );
    console.log("bal = ", bal);

    sig  = await program.rpc.closeLottery([token_mint], {
      accounts: {
          lottery: lotteryPDA,
          clockSysvar: SYSVAR_CLOCK_PUBKEY,
      },
    });
    res = await provider.connection.confirmTransaction(sig);
    console.log("confirmed tx = ", sig);

    let lotteryInfo = await program.account.lottery.fetch(lotteryPDA);
    console.log("lotteryInfo = ", lotteryInfo.winnerTicket.toBase58());

    sig = await program.rpc.claim({
      accounts: {
        lottery: lotteryPDA,
        owner: signer.publicKey,
        user: signer.publicKey,
        nftMint: token_mint,
        winnerTicket: token_mint,
        escrow: escrowPDA,
        vault: vaultPDA,
        ticketTokenAccount: ownerTokenAccount.address,
        receiveTokenAccount: ownerTokenAccount.address,
        clockSysvar: SYSVAR_CLOCK_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      },
    });
    res = await provider.connection.confirmTransaction(sig);
    console.log("confirmed tx = ", sig, res);

    ownerTokenAccountInfo = await getAccount(
      provider.connection,
      ownerTokenAccount.address,
    );
    console.log("ownerTokenAccountinfo.amount = ", ownerTokenAccountInfo.amount);

    bal = await provider.connection.getBalance(
      vaultPDA,
      "confirmed"
    );
    console.log("bal = ", bal);

  });
});
