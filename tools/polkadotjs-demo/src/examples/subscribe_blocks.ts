//! This is a simple example to subscribe to the new blocks and print the block number and hash.
//! It will keep on listening to the new blocks until the count reaches 10.

import { ApiPromise, WsProvider } from "@polkadot/api";

const main = async () => {
    const wsProvider = new WsProvider(
        "wss://polkadot.public.curie.radiumblock.co/ws"
    );
    const api = await ApiPromise.create({ provider: wsProvider });
    // Wait until we are ready and connected
    await api.isReady;

    // Retrieve the chain name, latest header
    const [chain, lastHeader] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.chain.getHeader(),
    ]);

    // Log the information
    console.log(
        `${chain}: last block #${lastHeader.number} has hash ${lastHeader.hash}`
    );

    let count = 1;
    console.log("count: ", count);

    let lastBlockNumber = lastHeader.number;

    // Subscribe to the new headers for a specific count or else it will keep on listening
    // NOTE: for this don't exit the process, we need to keep the listener open i.e. disable `process.exit(0)`
    const unsubHeads = await api.rpc.chain.subscribeNewHeads((lastHeader) => {
        if (count === 10) {
            unsubHeads();
        }

        const newBlockNumber = lastHeader.number;

        if (lastBlockNumber < newBlockNumber) {
            console.log(
                `${chain}: last block #${newBlockNumber} has hash ${lastHeader.hash}`
            );
            count++;
            lastBlockNumber = newBlockNumber;
            console.log("count: ", count);
        }
    });
};

main()
    .then(() => {
        /* process.exit(0) */
    })
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
