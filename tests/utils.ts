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
  mint: PublicKey,
  owner: PublicKey,
  allowOwnerOffCurve = false,
  tokenProgram: PublicKey
) {
  const ataPubkey = getAssociatedTokenAddressSync(
    mint,
    owner,
    allowOwnerOffCurve,
    tokenProgram
  );
  return ataPubkey;
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
  allowOwnerOffCurve = false
) {
  return customizedCreateAssociatedTokenAccount(
    owner,
    mintToken,
    connection,
    payerKeypair,
    allowOwnerOffCurve
  );
}

export async function customizedCreateAssociatedTokenAccount(
  owner: anchor.web3.PublicKey,
  mintToken: anchor.web3.PublicKey,
  connection: Connection,
  payerKeypair: Keypair,
  allowOwnerOffCurve = false
) {
  const tokenProgram = await getTokenProgramFromMint(mintToken, connection);
  console.log("TokenProgram:", tokenProgram.toBase58());
  let ataPubKey = find_ata_sync(
    mintToken,
    owner,
    allowOwnerOffCurve,
    tokenProgram
  );
  let accExist = await isAccountExist(connection, ataPubKey, false);
  if (accExist) {
    console.log("acc Exist is:", ataPubKey.toBase58());
    return ataPubKey;
  }
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

export async function getTokenProgramFromMint(
  mintPubkey: anchor.web3.PublicKey,
  connection: anchor.web3.Connection
) {
  // 获取 Mint 账户信息
  const accountInfo = await connection.getAccountInfo(mintPubkey);
  if (!accountInfo) {
    throw new Error(`Mint account ${mintPubkey} not found on chain`);
  }
  // 提取 owner 字段
  const mintOwner = accountInfo.owner;
  // console.log("Mint Owner:", mintOwner.toBase58());

  // 判断 TokenProgram
  if (mintOwner.equals(TOKEN_PROGRAM_ID)) {
    return TOKEN_PROGRAM_ID;
  } else if (mintOwner.equals(TOKEN_2022_PROGRAM_ID)) {
    return TOKEN_2022_PROGRAM_ID;
  } else {
    throw new Error(
      `Unknown Token Program for mint ${mintPubkey}: ${mintOwner.toBase58()}`
    );
  }
}
