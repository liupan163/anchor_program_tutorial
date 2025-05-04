import * as anchor from '@coral-xyz/anchor';
import {
  getMint,
  getTransferFeeConfig,
} from '@solana/spl-token';
import * as web3 from '@solana/web3.js';

import { getTokenProgramFromMint } from '../utils';

describe("sol-program", () => {
  const connection = new web3.Connection("https://api.mainnet-beta.solana.com","confirmed"); // todo
  //const connection = new web3.Connection("http://127.0.0.1:8899", "confirmed"); // todo
  // console.log("connection", connection);

  const owner = anchor.Wallet.local().payer;
  console.log("owner", owner.publicKey.toBase58());

  let pweaseMint = new web3.PublicKey(
    "CniPCE4b3s8gSUPhUiyMjXnytrEqUrMfSsnbBjLCpump"
  );
  let IPLRMint = new web3.PublicKey(
    "EyzgnBfHGe9hh169B8993muBVcoeURCnSgPbddBeSybo"
  );

  it("OP Operator Test  Sell Case without lookup table!", async () => {
    const mintPubkey = IPLRMint;
    console.log("mintPubkey", mintPubkey.toBase58());
    const tokenPorgram = await getTokenProgramFromMint(mintPubkey, connection);
    console.log("tokenPorgram", tokenPorgram.toBase58());
    // 获取铸造账户信息
    const mintInfo = await getMint(
      connection,
      mintPubkey,
      undefined,
      tokenPorgram
    );
    console.log("铸造账户:", mintInfo.address.toBase58());

    // 获取 TransferFeeConfig
    const transferFeeConfig = await getTransferFeeConfig(mintInfo);

    if (transferFeeConfig === null) {
      console.log("此铸造账户未启用 TransferFeeConfig 扩展");
      return;
    }

    // 提取费用信息
    const { newerTransferFee, olderTransferFee } = transferFeeConfig;
    const currentFeeBasisPoints = newerTransferFee.transferFeeBasisPoints;
    const maximumFee = newerTransferFee.maximumFee;
    const currentEpoch = newerTransferFee.epoch;

    // 计算百分比费率
    const feePercentage = currentFeeBasisPoints / 100; // 基点转为百分比

    // 输出结果
    console.log("转账费信息:");
    console.log(
      `  当前费率: ${currentFeeBasisPoints} 基点 (${feePercentage}%)`
    );
    console.log(`  最大费用: ${maximumFee.toString()} token 单位`);
    console.log(`  当前 Epoch: ${currentEpoch.toString()}`);
    console.log(
      `  配置权限: ${transferFeeConfig.transferFeeConfigAuthority.toBase58()}`
    );
    console.log(
      `  提取权限: ${transferFeeConfig.withdrawWithheldAuthority.toBase58()}`
    );
    console.log(
      `  扣留总额: ${transferFeeConfig.withheldAmount.toString()} token 单位`
    );

    if (olderTransferFee.epoch !== currentEpoch) {
      console.log("旧费用（已过期）:");
      console.log(`  费率: ${olderTransferFee.transferFeeBasisPoints} 基点`);
      console.log(`  最大费用: ${olderTransferFee.maximumFee.toString()}`);
    }
  });
});
