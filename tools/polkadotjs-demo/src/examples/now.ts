//! Get the current timestamp from the chain

import { ApiPromise, WsProvider } from "@polkadot/api";

const main = async () => {
    const wsProvider = new WsProvider(
        "wss://polkadot.public.curie.radiumblock.co/ws"
    );
    const api = await ApiPromise.create({ provider: wsProvider });
    // Wait until we are ready and connected
    await api.isReady;

    const now = await api.query.timestamp.now();
    console.log(now);
};

main()
    .catch(console.error)
    .finally(() => process.exit());

export default main;
