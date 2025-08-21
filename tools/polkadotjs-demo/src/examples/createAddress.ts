/**
 * Put this file inside `src/examples/` folder
 */
// for these, just import the `polkadot/api` package
import { Keyring } from "@polkadot/api";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { mnemonicGenerate } from "@polkadot/util-crypto";

async function createAddress(rpcApiUrl: string) {
    // Construct
    const wsProvider = new WsProvider(rpcApiUrl);
    const api = await ApiPromise.create({
        provider: wsProvider,
    });
    await api.isReady;

    // Create a keyring instance
    const keyring = new Keyring({ type: "sr25519" });

    // generate a random seed phrase & then put as Uri
    const mnemonic = mnemonicGenerate();
    const alice = keyring.addFromUri(mnemonic);
    // to use a specific keypair type, you can specify it as the second argument
    // const alice = keyring.addFromUri("//Alice", { name: "sr25519" }, "ed25519");
    console.log(
        `Alice has address ${alice.address} and public key: 0x${Array.from(
            alice.publicKey
        )
            .map((byte) => byte.toString(16).padStart(2, "0"))
            .join("")}}`
    );
}

async function main() {
    const rpcApiUrl = "wss://rpc.polkadot.io";
    await createAddress(rpcApiUrl).catch(console.error);
}

main()
    .catch(console.error)
    .finally(() => process.exit());
