import { ApiPromise, WsProvider } from "@polkadot/api";
import { AccountInfo } from "@polkadot/types/interfaces";

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
}

main()
    .then(() => {
        /* process.exit(0) */
    })
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
