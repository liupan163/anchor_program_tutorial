import {
  MARKET_STATE_LAYOUT_V2,
  publicKey,
  struct,
  u128,
  u64,
} from '@raydium-io/raydium-sdk-v2';
import {
  clusterApiUrl,
  Connection,
  PublicKey,
} from '@solana/web3.js';

let connection = new Connection(clusterApiUrl("mainnet-beta"));

export interface RawMint {
  mintAuthorityOption: 1 | 0;
  mintAuthority: PublicKey;
  supply: bigint;
  decimals: number;
  isInitialized: boolean;
  freezeAuthorityOption: 1 | 0;
  freezeAuthority: PublicKey;
}

// export const MintLayout = struct<RawMint>([
//   u32('mintAuthorityOption'),
//   publicKey('mintAuthority'),
//   u64('supply'),
//   u8('decimals'),
//   bool('isInitialized'),
//   u32('freezeAuthorityOption'),
//   publicKey('freezeAuthority'),
// ]);

export const StableAmmInfoLayout = struct([
  u64("status"),
  u64("nonce"),
  u64("orderNum"),
  u64("maxOrder"),
  u64("depth"),
  u64("baseDecimal"),
  u64("quoteDecimal"),
  u64("state"),
  u64("resetFlag"),
  u64("minSize"),
  u64("volMaxCutRatio"),
  u64("amountWaveRatio"),
  u64("baseLotSize"),
  u64("quoteLotSize"),
  u64("minPriceMultiplier"),
  u64("maxPriceMultiplier"),
  u64("systemDecimalsValue"),
  u64("abortTradeFactor"),
  u64("priceTickMultiplier"),
  u64("priceTick"),
  u64("minSeparateNumerator"),
  u64("minSeparateDenominator"),
  u64("tradeFeeNumerator"),
  u64("tradeFeeDenominator"),
  u64("pnlNumerator"),
  u64("pnlDenominator"),
  u64("swapFeeNumerator"),
  u64("swapFeeDenominator"),
  u64("baseNeedTakePnl"),
  u64("quoteNeedTakePnl"),
  u64("quoteTotalPnl"),
  u64("baseTotalPnl"),
  u64("poolOpenTime"),
  u64("punishPcAmount"),
  u64("punishCoinAmount"),
  u64("orderbookToInitTime"),

  u128("swapBaseInAmount"),
  u128("swapQuoteOutAmount"),
  u128("swapQuoteInAmount"),
  u128("swapBaseOutAmount"),
  u64("swapQuote2BaseFee"),
  u64("swapBase2QuoteFee"),

  publicKey("baseVault"),
  publicKey("quoteVault"),
  publicKey("baseMint"),
  publicKey("quoteMint"),
  publicKey("lpMint"),
  publicKey("modelDataAccount"),
  publicKey("openOrders"),
  publicKey("marketId"),
  publicKey("marketProgramId"),
  publicKey("targetOrders"),
  publicKey("owner"),
]);

async function getStableAmmPoolKeys(targetPoolAccount: PublicKey) {
  console.log(
    `Step - 1: Fetching Account Data for ${targetPoolAccount.toBase58()}`
  );
  let { data } = (await connection.getAccountInfo(targetPoolAccount)) || {};
  if (!data) return;

  console.log(`Step - 2: Deserializing Found Account Data`);
  const deserialized = StableAmmInfoLayout.decode(data);
  console.log(deserialized);
}

const layout = MARKET_STATE_LAYOUT_V2; // todo
console.log({ layout });

function main(): void {
  let targetPoolAccount = new PublicKey(
    "2EXiumdi14E9b8Fy62QcA5Uh6WdHS2b38wtSxp72Mibj"
  ); // Pool-USDT-USDC
  getStableAmmPoolKeys(targetPoolAccount).catch((error) => {
    console.error(
      "An error in getStableAmmPoolKeys process:",
      error
    );
  });
}

// ts-node tests/idl_
main();
