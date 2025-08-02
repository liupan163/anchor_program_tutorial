import * as bip39 from 'bip39';
import { derivePath } from 'ed25519-hd-key';

import * as anchor from '@coral-xyz/anchor';
import {
  createAssociatedTokenAccountInstruction,
  createSyncNativeInstruction,
  getAssociatedTokenAddress,
  getAssociatedTokenAddressSync,
  NATIVE_MINT,
  TOKEN_2022_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from '@solana/web3.js';

export function getPdaAddress(
  seeds: Buffer[],
  programId: PublicKey
): { pdaAddress: PublicKey; bump: number } {
  const [pdaAddress, bump] = PublicKey.findProgramAddressSync(seeds, programId);
  return { pdaAddress, bump };
}

export function getAtaSync(
  mint: PublicKey,
  owner: PublicKey,
  tokenProgram: PublicKey,
  allowOwnerOffCurve = false
) {
  const ataPubkey = getAssociatedTokenAddressSync(
    mint,
    owner,
    allowOwnerOffCurve,
    tokenProgram
  );
  return ataPubkey;
}

export async function isAccountExist(
  connection: anchor.web3.Connection,
  account: anchor.web3.PublicKey,
  isConsoleOut = false
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

export async function createWSOLTokenAccountAndSyncNativeInstructions(
  connection: Connection,
  owner: PublicKey,
  payer: PublicKey,
  syncAmount: bigint,
  allowOwnerOffCurve = false
): Promise<{ ata: PublicKey; instructions: TransactionInstruction[] }> {
  let { ata, instruction } = await createAtaIx(
    connection,
    payer,
    NATIVE_MINT,
    owner,
    allowOwnerOffCurve
  );
  const instructions = [instruction];

  let lamports;

  if (instruction) {
    lamports = BigInt(syncAmount);
  } else {
    let balance = await connection.getTokenAccountBalance(ata);
    let existingBalance = BigInt(balance.value.amount ?? "0");
    lamports = syncAmount - existingBalance;
  }
  if (lamports > 0) {
    // need transfer more SOL to WSOL
    instructions.push(
      SystemProgram.transfer({
        fromPubkey: payer,
        toPubkey: ata,
        lamports,
      }),
      createSyncNativeInstruction(ata)
    );
  }

  return {
    ata,
    instructions: instructions.filter(Boolean),
  };
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

export async function createAtaIx(
  connection: Connection,
  payer: PublicKey, // 支付 rent + 创建费
  targetMint: PublicKey, // 用户的 owner address
  owner: PublicKey,
  allowOwnerOffCurve = false
): Promise<{
  ata: PublicKey;
  createAtaIx?: TransactionInstruction;
  instruction?: TransactionInstruction;
}> {
  const mintTokenProgram = await getTokenProgramFromMint(
    targetMint,
    connection
  );
  const ata = await getAssociatedTokenAddress(
    targetMint,
    owner,
    allowOwnerOffCurve,
    mintTokenProgram
  );
  if (await isAccountExist(connection, ata, false)) {
    // Skip createAtaIx
    return {
      ata,
    };
  }

  // 构造创建 Associated Token Account 的指令（Instruction）
  const createAtaIx = createAssociatedTokenAccountInstruction(
    payer, // 付款人（必须 signer）
    ata, // 目标 token account（PDA）
    owner, // owner：此 ATA 属于谁
    targetMint, // token mint
    mintTokenProgram // token program
  );

  return {
    ata,
    createAtaIx,
    instruction: createAtaIx,
  };
}

export async function getKeypairFromMnemonic(mnemonic: string, derivationPath = "m/44'/501'/0'/0'"){
  // 1. 验证助记词
  if (!bip39.validateMnemonic(mnemonic)) {
    throw new Error('Invalid mnemonic');
  }

  // 2. 助记词 -> seed
  const seed = await bip39.mnemonicToSeed(mnemonic);

  // 3. 派生路径（默认 m/44'/501'/0'/0' 是 Solana 标准）
  const derivedSeed = derivePath(derivationPath, seed.toString('hex')).key;

  // 4. 从派生的 seed 生成 Solana Keypair
  return Keypair.fromSeed(derivedSeed);
};