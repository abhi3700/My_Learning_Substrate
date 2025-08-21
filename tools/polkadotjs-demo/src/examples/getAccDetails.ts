/**
 * Put this file inside `src/examples/` folder
 */

import { ApiPromise, WsProvider } from "@polkadot/api";
import { AccountInfo } from "@polkadot/types/interfaces";
import { POLKADOT_RPC_API_URL } from "./constants";

async function getAccountDetails(address: string) {
    const provider = new WsProvider(POLKADOT_RPC_API_URL);
    const api = await ApiPromise.create({ provider });
    await api.isReady;

    // Query the account information and cast to the expected type
    const accountInfo = (await api.query.system.account(
        address
    )) as unknown as AccountInfo;

    const nonce = accountInfo.nonce.toNumber();

    if (
        nonce === 0 &&
        accountInfo.data.free.toNumber() === 0 &&
        accountInfo.data.reserved.toNumber() === 0
    ) {
        console.log(`The account ${address} does not exist on the chain`);
    }

    await api.disconnect();
}

async function main() {
    const address = "5HgGWs9XZ5DYzM6mzxFeLkCne6tP4T1rMZtya4KNBt6FMsqm"; // Replace with the actual address
    await getAccountDetails(address).catch(console.error);
}

main()
    .catch(console.error)
    .finally(() => process.exit());
