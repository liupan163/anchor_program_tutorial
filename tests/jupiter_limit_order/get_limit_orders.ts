import axios from 'axios';

type GetOpenLimitOrders = {
    wallet: string,
    inputMint: string,
    outputMint: string,
}

type HistoryOrder = {
    userPubkey: string,
    orderKey: string,
    inputMint: string,
    outputMint: string,
    makingAmount: string,
    takingAmount: string,
    remainingMakingAmount: string,
    remainingTakingAmount: string,
    expiredAt: null,
    createdAt: string,
    updatedAt: string,
    status: string,
    openTx: string,
    closeTx: string,
    programVersion: string,
    trades: []
};

type HistoryOrdersResponse = {
    orders: HistoryOrder[];
};

type OpenLimitAccount = {
    borrowMakingAmount: Number,
    createdAt: Number,
    expiredAt: Number,
    makingAmount: Number,
    oriMakingAmount: Number,
    oriTakingAmount: Number,
    takingAmount: Number,
    uniqueId: String,
    updatedAt: Number,
    feeAccount: String,
    inputMint: String,
    inputMintReserve: String,
    inputTokenProgram: String,
    maker: String,
    outputMint: String,
    outputTokenProgram: String,
    feeBps: Number,
    bump: Number
}
type OpenLimitOrder = {
    account: OpenLimitAccount,
    publicKey: string,
}
type OpenLimitOrdersResponse = OpenLimitOrder[]

export async function GetOpenLimitOrders(walletPubkey: string) {
    const getOpenOrdersBody: GetOpenLimitOrders = {
        wallet: walletPubkey,
        inputMint: "So11111111111111111111111111111111111111112",
        outputMint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    };

    const response = await axios.get("https://api.jup.ag/limit/v2/openOrders?wallet=" + getOpenOrdersBody.wallet +
        "&inputMint=" + getOpenOrdersBody.inputMint + "&outputMint=" + getOpenOrdersBody.outputMint);

    // console.log("response ->",response)
    const getOrdersResponse: OpenLimitOrdersResponse = response.data;
    // console.log("orders ->", getOrdersResponse);
    return getOrdersResponse;
}

