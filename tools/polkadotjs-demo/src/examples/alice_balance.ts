//! Any substrate relay chain with balances pallet can be kickstarted in dev mmode.
//! And at genesis level, if there are balances in genesis account - Alice, Bob, Charlie, etc.

import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { encodeAddress } from "@polkadot/keyring";
import { AccountInfo } from "@polkadot/types/interfaces";

async function balanceOf(api: ApiPromise, address: string) {
    const accountInfo = (await api.query.system.account(
        address
    )) as unknown as AccountInfo;

    return accountInfo.data.free.toString();
}

async function main() {
    // connect to the local node
    const wsProvider = new WsProvider("ws://127.0.0.1:9944");
    const api = await ApiPromise.create({ provider: wsProvider });
    // Wait until we are ready and connected
    await api.isReady;

    console.log(`Genesis hash: ${api.genesisHash.toHex()}`);
    console.log(`Runtime spec name: ${api.runtimeVersion.specName.toString()}`);
    console.log(
        `Runtime spec version: ${api.runtimeVersion.specVersion.toString()}`
    );

    // Initialize the signer keypair
    const keyring = new Keyring({ type: "sr25519" });
    const alice = keyring.createFromUri("//Alice");
    console.log(`Alice's SS58 Address: ${alice.address}`);
    console.log(
        `Alice's public key: 0x${Array.from(alice.publicKey)
            .map((byte) => byte.toString(16).padStart(2, "0"))
            .join("")}`
    );

    // subspace address by encoding with no. 2254
    const ssAddress = encodeAddress(alice.address, 2254);
    console.log(`Alice's SS Address: ${ssAddress}`);
    console.log(`Alice's free balance: ${balanceOf(api, alice.address)}`);

    // TODO: check the balance of the genenis account.
}

main()
    .then(() => {
        process.exit(0);
    })
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
