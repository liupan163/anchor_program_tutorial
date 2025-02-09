import axios from 'axios';

type CreateOrder = {
    inputMint: string;
    outputMint: string;
    maker: string;
    payer: string;
    params: {
        makingAmount: string;
        takingAmount: string;
        expiredAt?: string;
        feeBps?: string;
    };
    computeUnitPrice: string | "auto";
    referral?: string;
    inputTokenProgram?: string;
    outputTokenProgram?: string;
    wrapAndUnwrapSol?: boolean;
};

type CreateOrderResponse = {
    order: string;
    tx: string;
};

export async function CreateLimitOrder(walletPubkey: string) {
    try {
        const createOrderBody: CreateOrder = {
            //TODO:填充地址和payer，保持一致
            maker: walletPubkey,
            payer: walletPubkey,
            inputMint: "So11111111111111111111111111111111111111112", // WSOL
            outputMint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
            params: {
                makingAmount: "22000000", // 0.022 SOL
                takingAmount: "5258000", // 5.258 USDC     =  239$ *0.022
            },
            // "auto" sets the priority fee based on network congestion
            // and it will be capped at 500,000
            computeUnitPrice: "auto",
        };

        const response = await axios.post("https://api.jup.ag/limit/v2/createOrder",
            JSON.stringify(createOrderBody), {
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json',
            },
        });
        
        // console.log("creter limit order response=>", response)
        const createOrderResponse: CreateOrderResponse = response.data;
        return createOrderResponse.tx;
    } catch (error) {
        throw error
    }
}
