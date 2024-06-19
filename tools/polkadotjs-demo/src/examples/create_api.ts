/**
 * Put this file inside `src/examples/` folder
 * Create an API instance for the Substrate relay chain
 */

import { ApiPromise, WsProvider } from "@polkadot/api";
import { connectApi } from "./connect_api";

export async function createSubstrateApiInstance(
    // starts with wss:// or ws://
    rpcApiUrl: string
): Promise<ApiPromise> {
    // Construct
    const wsProvider = new WsProvider(rpcApiUrl);
    const api = await ApiPromise.create({
        provider: wsProvider,
    });
    return api;
}

async function main() {
    const rpcApiUrl = "wss://rpc.polkadot.io";
    await connectApi(rpcApiUrl).catch(console.error);
}

main()
    .catch(console.error)
    .finally(() => process.exit());
