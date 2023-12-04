/**
 * Put this file inside `src/examples/` folder
 */
import { createSubstrateApiInstance } from "./create_api";
// for these, just import the `polkadot/api` package
import { Keyring } from "@polkadot/api";
import { mnemonicGenerate } from "@polkadot/util-crypto";

async function createAddress(rpcApiUrl: string) {
  // Import the keyring as required

  // Initialize the API as we would normally do
  const api = await createSubstrateApiInstance(rpcApiUrl);

  // Create a keyring instance
  const keyring = new Keyring({ type: "sr25519" });

  // generate a random seed phrase & then put as Uri
  const mnemonic = mnemonicGenerate();
  const alice = keyring.addFromUri(mnemonic);
  console.log(
    `Alice has address ${alice.address} and public key: ${alice.publicKey}}`
  );
}

async function main() {
  const rpcApiUrl = "wss://rpc.polkadot.io";
  await createAddress(rpcApiUrl).catch(console.error);
}

main()
  .catch(console.error)
  .finally(() => process.exit());
