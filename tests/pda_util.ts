import {
    Connection,
    ConfirmOptions,
    PublicKey,
    Keypair,
    Signer,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    ComputeBudgetInstruction,
    ComputeBudgetProgram,
    Transaction,
} from "@solana/web3.js";
import {
    TOKEN_PROGRAM_ID,
    TOKEN_2022_PROGRAM_ID,
    getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import * as anchor from "@coral-xyz/anchor";

function find_pda_address(seeds: Array<Buffer | Uint8Array>, programId: PublicKey) {
    const [pdaPubkey] = PublicKey.findProgramAddressSync(
        // [Buffer.from(POOL_CONFIG_SEED)],
        seeds,
        programId
    );
    console.log("pdaPubkey is:", pdaPubkey.toBase58())
}

function find_ata_sync(token0: PublicKey, creatorPubkey: PublicKey, programId: PublicKey, tokenProgram: PublicKey) {
    const ataPubkey = getAssociatedTokenAddressSync(
        token0,
        creatorPubkey,
        false,
        tokenProgram,
    );
    console.log("ataPubkey is:", ataPubkey.toBase58())
}


function main() {
    let testPubkey = new PublicKey("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK");
    let poolPubkey = new PublicKey("ExcBWu8fGPdJiaF1b1z3iEef38sjQJks8xvj6M85pPY6");
    find_pda_address(
        [
            Buffer.from("pool_tick_array_bitmap_extension"), 
            poolPubkey.toBuffer()
        ],
        testPubkey
    ); // CAMM,  extensionAccount

    find_pda_address(
        [
            Buffer.from("pool"), 
            (new PublicKey("EdPxg8QaeFSrTYqdWJn6Kezwy9McWncTYueD9eMGCuzR")).toBuffer(),
            (new PublicKey("So11111111111111111111111111111111111111112")).toBuffer(),
            (new PublicKey("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB")).toBuffer(),
        ],
        new PublicKey("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK")
    ); // output: ExcBWu8fGPdJiaF1b1z3iEef38sjQJks8xvj6M85pPY6
    find_pda_address(
        [
            Buffer.from("tick_array"), 
            (new PublicKey("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB")).toBuffer(),
            new anchor.BN(4509720).toArrayLike(Buffer, "be", 8),
        ],
        new PublicKey("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK")
    );
    console.log("main exit ...")
}

main(); // ts-node tests/pda_utils.ts

// --- --- ------ --- ------ --- ------ --- ------ --- ------ --- ------ --- ------ --- ------ --- ---
export function u16ToBytes(num: number) {
    const arr = new ArrayBuffer(2);
    const view = new DataView(arr);
    view.setUint16(0, num, false);
    return new Uint8Array(arr);
}

export function i16ToBytes(num: number) {
    const arr = new ArrayBuffer(2);
    const view = new DataView(arr);
    view.setInt16(0, num, false);
    return new Uint8Array(arr);
}

export function u32ToBytes(num: number) {
    const arr = new ArrayBuffer(4);
    const view = new DataView(arr);
    view.setUint32(0, num, false);
    return new Uint8Array(arr);
}

export function i32ToBytes(num: number) {
    const arr = new ArrayBuffer(4);
    const view = new DataView(arr);
    view.setInt32(0, num, false);
    return new Uint8Array(arr);
}
