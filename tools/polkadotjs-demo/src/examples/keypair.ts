import { ApiPromise, WsProvider } from "@polkadot/api";
import { POLKADOT_RPC_API_URL } from "./constants";

async function main(): Promise<void> {
    // Construct
    const wsProvider = new WsProvider(POLKADOT_RPC_API_URL);
    const api = await ApiPromise.create({
        provider: wsProvider,
    });
    await api.isReady;
}

main()
    .catch(console.error)
    .finally(() => process.exit());
