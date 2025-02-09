import {
    PublicKey,
    LAMPORTS_PER_SOL,
    Transaction,
    VersionedTransaction,
} from '@solana/web3.js';
import { RaydiumSwap } from './raydium-swap';
import { CONFIG } from './config';

async function fetch_pool_info() {
    console.log('Starting fetch_pool_info process...');
    const raydiumSwap = new RaydiumSwap(CONFIG.RPC_URL, CONFIG.WALLET_SECRET_KEY);

    await raydiumSwap.loadPoolKeys();
    let poolInfo = raydiumSwap.findPoolInfoForTokens(CONFIG.BASE_MINT, CONFIG.QUOTE_MINT)
        || await raydiumSwap.findRaydiumPoolInfo(CONFIG.BASE_MINT, CONFIG.QUOTE_MINT);

    if (!poolInfo) {
        throw new Error("Couldn't find the pool info");
    }

    // await raydiumSwap.createWrappedSolAccountInstruction(CONFIG.TOKEN_A_AMOUNT);

}

fetch_pool_info().catch((error) => {
    console.error('An error occurred during the fetch_pool_info process:');
    console.error(error);
});