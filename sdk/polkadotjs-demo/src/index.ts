import { ApiPromise, WsProvider } from "@polkadot/api";

// Define the interface for the account info as JSON structure
interface AccountInfo {
    nonce: number;
    consumers: number;
    providers: number;
    sufficients: number;
    data: {
        free: string;
        reserved: string;
        frozen: string;
        flags: string;
    };
}

async function main() {
    const wsProvider = new WsProvider(
        "wss://polkadot.public.curie.radiumblock.co/ws"
    );
    const api = await ApiPromise.create({ provider: wsProvider });
    // Wait until we are ready and connected
    await api.isReady;

    console.log(api.genesisHash.toHex());
    console.log(api.runtimeVersion.specName.toString());
    console.log(api.runtimeVersion.specVersion.toString());

    console.log(api.consts.babe.epochDuration.toString());
    console.log(api.consts.balances.existentialDeposit.toString());

    // The actual address that we will use
    const ADDR = "16HNPJqej7ECnB4xQWEk1jAgjpsC65pRjAEXoznmQFxxL8cj";

    // Retrieve the account balance & nonce via the system module
    const accountInfo = await api.query.system.account(ADDR);
    console.log("Full account info: ", accountInfo.toJSON());

    const [now, { nonce, data: balance }] = await Promise.all([
        api.query.timestamp.now(),
        api.query.system.account(ADDR) as unknown as Promise<AccountInfo>,
    ]);
    console.log(`${now}: balance of ${balance.free} and a nonce of ${nonce}`);

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
}

main()
    .then(() => {
        /* process.exit(0) */
    })
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
