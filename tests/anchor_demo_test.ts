import { assert } from 'chai';

import * as anchor from '@coral-xyz/anchor';
import {
  AnchorError,
  Program,
} from '@coral-xyz/anchor';

import { AnchorDemo } from '../target/types/anchor_demo';

// solana airdrop 5 GTx4x6mqoNydgvVUvHfhenTCoCyhdKZ9gzYtY4zRMLzf

describe("anchor_demo_1", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // Change this to be your programID
  const programID = "HnWGvRLFS2P6wNxAuGW8Kmm3T1e6ztvKBJNBTnhPcP4i";
  /*
    let idl = JSON.parse(fs.readFileSync('target/idl/anchor_demo_1.json', 'utf-8'))
    const program = new Program(idl, programID, anchor.getProvider());
  */
  const program = anchor.workspace.AnchorDemo as Program<AnchorDemo>;

  const defaultKeyPair = new anchor.web3.PublicKey(
    // replace this with your default provider keypair, you can get it by running `solana address` in your terminal
    "GTx4x6mqoNydgvVUvHfhenTCoCyhdKZ9gzYtY4zRMLzf"
  );

  // generate a signer to call our function
  let myKeypair = anchor.web3.Keypair.generate();

  /*
      Tips：
      1. Anchor 悄悄地将 Rust 的蛇形命名法, 转换为 Typescript 的驼峰命名法
  */

  it("Is initialized!", async () => {
    const seeds = [];
    const [myStorage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
      seeds,
      program.programId
    );

    // Add your test here.
    console.log("enter initialized->");

    let bal_before = await program.provider.connection.getBalance(
      defaultKeyPair
    );
    console.log("before:", bal_before);

    // @ts-ignore
    const tx = await program.methods
      .initialize()
      .accounts({
        signer1: program.provider.publicKey,
        signer2: myKeypair.publicKey,
      })
      .signers([myKeypair])
      .rpc();

    console.log("Your transaction signature", tx);

    // log the keypair's balance after
    let bal_after = await program.provider.connection.getBalance(
      defaultKeyPair
    );
    console.log("after:", bal_after);

    // log the difference
    console.log(
      "diff:",
      BigInt(bal_before.toString()) - BigInt(bal_after.toString())
    );

    console.log("The signer1: ", program.provider.publicKey.toBase58()); // NB: check it's ur wallet address
    console.log("The signer2: ", myKeypair.publicKey.toBase58());

    const listenerMyEvent = program.addEventListener(
      "MyEvent",
      (event, slot) => {
        console.log(`slot ${slot} event value ${event.value}`);
      }
    );
  });

  it("error handle demo", async () => {
    try {
      const tx = await program.methods.limitRange(new anchor.BN(9)).rpc();
      console.log("Your transaction signature 9", tx);
    } catch (_err) {
      assert.isTrue(_err instanceof AnchorError);
      const err: AnchorError = _err;
      const errMsg = "a is too small";
      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log("Error number:", err.error.errorCode.number);
    }

    try {
      const tx = await program.methods.limitRange(new anchor.BN(101)).rpc();
      console.log("Your transaction signature 101", tx);
    } catch (_err) {
      assert.isTrue(_err instanceof AnchorError);
      const err: AnchorError = _err;
      const errMsg = "a is too big";
      assert.strictEqual(err.error.errorMessage, errMsg);
      console.log("Error number:", err.error.errorCode.number);
    }
  });

  it("Is called by the owner", async () => {
    // Add your test here.
    const tx = await program.methods
      .checkIdentity()
      .accounts({
        signerAccount: program.provider.publicKey,
      })
      .rpc();

    console.log("Transaction hash:", tx);
  });
});
