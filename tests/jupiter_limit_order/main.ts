import {
    Connection,
    Keypair,
    SendOptions,
    VersionedTransaction,
} from "@solana/web3.js";
import { CreateLimitOrder } from "./create_limit_order"
import { GetOpenLimitOrders } from "./get_limit_orders"
import { CancelLimitOrder } from "./cancel_limit_order"
import bs58 from "bs58";
import dotenv from 'dotenv';
dotenv.config();

const WALLET_PRIV_KEY = process.env.WALLET_PRIV_KEY;
if (!WALLET_PRIV_KEY) throw "missing WALLET_PRIV_KEY";

const wallet = Keypair.fromSecretKey(bs58.decode(WALLET_PRIV_KEY));
console.log('Wallet initialized with public key:', wallet.publicKey.toBase58());

const RPC_URL = process.env.RPC_URL; //TODO: 修改RPC URL
if (!RPC_URL) throw "missing RPC_URL env var";

const RPC_CONNECTION = new Connection(RPC_URL);

async function createLimitOrder() {
    try {
        const createOrderData = await CreateLimitOrder(wallet.publicKey.toBase58());
        console.log("createOrderData ->", createOrderData);
        const txBuff = Buffer.from(createOrderData, "base64");
        const vtx = VersionedTransaction.deserialize(txBuff);

        // sign with wallet
        vtx.sign([wallet]);

        const rpcSendOpts: SendOptions = { skipPreflight: true };
        const hash = await RPC_CONNECTION.sendRawTransaction(
            vtx.serialize(),
            rpcSendOpts
        );
        console.log("hash->", hash);
        return hash
    } catch (error) {
        console.error('An error occurred:', error);
    }
}

async function getOpenLimitOrders() {
    try {
        const getOpenLimitOrdersResponse = await GetOpenLimitOrders(wallet.publicKey.toBase58());
        console.log("getOpenLimitOrdersResponse ->", getOpenLimitOrdersResponse);
    } catch (error) {
        console.error('An error occurred:', error);
    }
}

async function cancel_limit_order(orderId: string) {
    try {
        const cancelOrdersResponse = await CancelLimitOrder(wallet.publicKey.toBase58(), orderId);
        console.log("cancelOrdersResponse ->", cancelOrdersResponse);

        const txBuff = Buffer.from(cancelOrdersResponse, "base64");
        const vtx = VersionedTransaction.deserialize(txBuff);

        // sign with wallet
        vtx.sign([wallet]);

        const rpcSendOpts: SendOptions = { skipPreflight: true };
        const hash = await RPC_CONNECTION.sendRawTransaction(
            vtx.serialize(),
            rpcSendOpts
        );
        console.log("hash ->", hash);
    } catch (error) {
        console.error('An error occurred:', error);
    }
}


async function main() {
    // await createLimitOrder();

    await getOpenLimitOrders();

    //let orderId = "8J7FG6a13vVwnwvm5URpggoYiyicAmu4XNsaRGhujpxR"; // todo replace
    //await cancel_limit_order(orderId);
}

// 调用主函数
main();