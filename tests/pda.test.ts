import {
  assert,
  expect,
} from 'chai';

import {
  Keypair,
  PublicKey,
} from '@solana/web3.js';

export function findProgramAddress(
  seeds: Array<Buffer | Uint8Array>,
  programId: PublicKey
): {
  publicKey: PublicKey;
  nonce: number;
} {
  const [publicKey, nonce] = PublicKey.findProgramAddressSync(seeds, programId);
  return { publicKey, nonce };
}

function generateSolDexCA() {
  const { publicKey: okxGenerateSolDexCA, nonce } = findProgramAddress(
    [Buffer.from("okx_sa")],
    new PublicKey("6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma") // OKX solDex Program
  );
  console.log("okxGenerateSolDexCA", okxGenerateSolDexCA.toBase58());
  console.log("nonce", nonce);
  expect(okxGenerateSolDexCA.toBase58()).to.equal(
    "HV1KXxWFaSeriyFvXyx48FqG9BoFbfinB8njCJonqP7K"
  ); // authority_pda Account:
}
function walletAddress() {
  let testAccKeypair = Keypair.fromSecretKey(
    Uint8Array.from([
      30, 63, 147, 208, 134, 13, 218, 88, 45, 252, 51, 208, 133, 2, 9, 85, 194,
      36, 66, 190, 10, 239, 220, 220, 243, 245, 127, 64, 254, 6, 183, 233, 195,
      145, 181, 66, 11, 19, 225, 136, 168, 83, 1, 1, 22, 18, 98, 235, 51, 5, 84,
      138, 143, 125, 174, 91, 252, 11, 190, 160, 121, 231, 17, 39,
    ])
  );
  assert.equal(
    testAccKeypair.publicKey.toBase58(),
    "EARNECPU8ZpkpheFz1wSqzn5YEcbwGpDVSFrNsJsg8Uv"
  );
  console.log("address", testAccKeypair.publicKey.toBase58());
  let dexSolanaKeypair = Keypair.fromSecretKey(
    Uint8Array.from([
      194, 196, 176, 70, 103, 25, 254, 166, 205, 232, 46, 164, 1, 4, 19, 4, 129,
      85, 8, 139, 53, 127, 89, 171, 219, 0, 217, 157, 200, 207, 51, 181, 203,
      28, 85, 144, 4, 137, 42, 247, 43, 149, 19, 97, 238, 216, 49, 183, 50, 242,
      15, 127, 195, 12, 154, 105, 22, 188, 29, 75, 10, 233, 177, 124,
    ])
  );
  assert.equal(
    testAccKeypair.publicKey.toBase58(),
    "EfrpeoLL9N6a6EkA4kcb8hmDe7bq8iiVN6mP21oMFG8T"
  );
  console.log("address", dexSolanaKeypair.publicKey.toBase58());
}
function main() {
  generateSolDexCA();
  walletAddress();
}
// ts-node tests/pda
main();
