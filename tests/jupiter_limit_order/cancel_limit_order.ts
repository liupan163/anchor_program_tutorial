import axios from 'axios';

type CancelOrders = {
    maker: string;

    // "auto" sets the priority fee based on network congestion
    // and it will be capped at 500,000
    computeUnitPrice: string | "auto";

    // Specific order account public keys to cancel/close
    orders?: string[] | undefined;
};

type CancelOrdersResponse = {
    txs: string[];
};

export async function CancelLimitOrder(walletPubkey :string, order: string){
    const cancelOrdersBody: CancelOrders = {
        maker: walletPubkey,
        // "auto" sets the priority fee based on network congestion
        // and it will be capped at 500,000
        computeUnitPrice: "auto",
        // 这里的order 值创建的时候会返回对应信息
        orders: [order]
    };

    const response = await axios.post("https://api.jup.ag/limit/v2/cancelOrders",
        JSON.stringify(cancelOrdersBody), {
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json',
        },
    });

    // console.log(response)
    const cancelOrdersResponse: CancelOrdersResponse = response.data;
    console.log("cancelOrdersResponse.txs->", cancelOrdersResponse.txs);
    if (cancelOrdersResponse.txs.length >= 1) {
        return cancelOrdersResponse.txs[0];
    }
}

