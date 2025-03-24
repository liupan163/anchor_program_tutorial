import * as anchor from '@coral-xyz/anchor';
import {
  createAssociatedTokenAccountInstruction,
  createSyncNativeInstruction,
  getAssociatedTokenAddress,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  TOKEN_2022_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
} from '@solana/web3.js';

export function getPdaAddress(
  seeds: Buffer[],
  programId: PublicKey
): {pdaAddress:PublicKey, bump:number} {
  const [pdaAddress, bump] = PublicKey.findProgramAddressSync(seeds, programId);
  return {pdaAddress, bump};
}

export function find_ata_sync(
  token0: PublicKey,
  creatorPubkey: PublicKey,
  programId: PublicKey,
  tokenProgram: PublicKey
) {
  const ataPubkey = getAssociatedTokenAddressSync(
    token0,
    creatorPubkey,
    false,
    tokenProgram
  );
  console.log("ataPubkey is:", ataPubkey.toBase58());
}

export async function buildCreateAssociatedTokenAccountTransaction(
  payer: PublicKey,
  mint: PublicKey,
  owner: PublicKey
): Promise<Transaction> {
  const associatedToken = await getAssociatedTokenAddress(mint, owner, false);
  console.log("inside buildCreate aTA:", associatedToken.toBase58());
  const transaction = new Transaction().add(
    createAssociatedTokenAccountInstruction(payer, associatedToken, owner, mint)
  );

  return transaction;
}

export async function isAccountExist(
  connection: anchor.web3.Connection,
  account: anchor.web3.PublicKey,
  isConsoleOut: boolean
) {
  const info = await connection.getAccountInfo(account);
  if (info == null || info.data.length == 0) {
    return false;
  }
  if (isConsoleOut) {
    console.log("account address:", account.toBase58());
    console.log("account info is:", info);
  }
  return true;
}

export async function createAssociatedSplTokenAccount(
  owner: anchor.web3.PublicKey,
  mintToken: anchor.web3.PublicKey,
  connection: Connection,
  payerKeypair: Keypair,
  allowOwnerOffCurve = false,
) {
  return createAssociatedTokenAccount(
    owner,
    mintToken,
    connection,
    payerKeypair,
    TOKEN_PROGRAM_ID,
    allowOwnerOffCurve
  );
}

export async function createAssociatedSpl2020TokenAccount(
  owner: anchor.web3.PublicKey,
  mintToken: anchor.web3.PublicKey,
  connection: Connection,
  payerKeypair: Keypair,
  allowOwnerOffCurve = false,
) {
  // 指定 Token2020 Program ID
  return createAssociatedTokenAccount(
    owner,
    mintToken,
    connection,
    payerKeypair,
    TOKEN_2022_PROGRAM_ID,
    allowOwnerOffCurve
  );
}

async function createAssociatedTokenAccount(
  owner: anchor.web3.PublicKey,
  mintToken: anchor.web3.PublicKey,
  connection: Connection,
  payerKeypair: Keypair,
  tokenProgram: PublicKey,
  allowOwnerOffCurve = false,
) {
  // console.log("TokenProgram:", tokenProgram.toBase58());
  const associatedTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    payerKeypair,
    mintToken,
    owner,
    allowOwnerOffCurve,
    "confirmed",
    {},
    tokenProgram
  );
  // console.log("ataAccount:", associatedTokenAccount.address.toBase58());
  return associatedTokenAccount.address;
}

export async function createWsolTokenAccountAndSyncNative(
  owner: anchor.web3.PublicKey,
  mintToken: anchor.web3.PublicKey,
  connection: Connection,
  payerKeypair: Keypair,
  syncAmount: number,
  allowOwnerOffCurve = false,
) {
  let wsolTokenAccount = await createAssociatedSplTokenAccount(
    owner,
    mintToken,
    connection,
    payerKeypair,
    allowOwnerOffCurve
  );
  const unifiedTransaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: payerKeypair.publicKey,
      toPubkey: wsolTokenAccount,
      lamports: syncAmount,
    }),
    createSyncNativeInstruction(wsolTokenAccount)
  );
  await sendAndConfirmTransaction(connection, unifiedTransaction, [
    payerKeypair,
  ]);
  let balanceResp = await connection.getTokenAccountBalance(wsolTokenAccount);
  console.log("wsol Balance:", balanceResp.value.amount);
  return wsolTokenAccount;
}
