import { web3 } from '@coral-xyz/anchor';
import {
  blob,
  struct,
  u8,
} from '@solana/buffer-layout';
import {
  publicKey,
  u64,
} from '@solana/buffer-layout-utils';
import { PublicKey } from '@solana/web3.js';

const OperatorWalletLayout = struct<{
  wallet_pubkey: PublicKey;
  start_slot: bigint;
  buy_order_capacity: bigint;
}>([
  publicKey("wallet_pubkey"),
  u64("start_slot"),
  u64("buy_order_capacity"),
]);

export const DEFAULT_OPERATOR_COUNT = 100;

const OPERATOR_WALLET_SIZE = OperatorWalletLayout.span;

export const AuthStoreLayout = struct<{
  admin: PublicKey;
  safe_withdraw_address: PublicKey;
  operator_wallet_list_bytes: Uint8Array;
  is_paused: number;
  is_locked_swap: number;
}>([
  publicKey('admin'),
  publicKey('safe_withdraw_address'),
  blob(OPERATOR_WALLET_SIZE * DEFAULT_OPERATOR_COUNT, 'operator_wallet_list_bytes'),
  u8('is_paused'),
  u8('is_locked_swap'),
]);

export function decodeAuthStore(data: Buffer) {
  const decoded = AuthStoreLayout.decode(data);

  const operator_wallet_list = [];
  for (let i = 0; i < DEFAULT_OPERATOR_COUNT; i++) {
    const offset = i * OPERATOR_WALLET_SIZE;
    const walletData = decoded.operator_wallet_list_bytes.slice(offset, offset + OPERATOR_WALLET_SIZE);
    const wallet = OperatorWalletLayout.decode(walletData);
    operator_wallet_list.push(wallet);
  }

  return {
    admin: decoded.admin,
    safe_withdraw_address: decoded.safe_withdraw_address,
    operator_wallet_list,
    is_paused: decoded.is_paused === 1,
    is_locked_swap: decoded.is_locked_swap === 1,
  };
}

async function main() {   
  // const connection = new web3.Connection("http://127.0.0.1:8899", "confirmed"); // todo
  const connection = new web3.Connection("https://api.mainnet-beta.solana.com", "confirmed");
  const authStorePubkey = new PublicKey("");

  const accountInfo = await connection.getAccountInfo(authStorePubkey);
  if (!accountInfo?.data) return;

  // const parsed = decodeAuthStore(accountInfo.data);
  const parsed = decodeAuthStore(accountInfo.data.slice(8)); // TODO keyPoint
  console.log("parsed:", parsed);
  // console.log("Admin:", parsed.admin.toBase58());
  // console.log("Operators:", parsed.operator_wallet_list);
}
main().then(() => {
  console.log("done");
});