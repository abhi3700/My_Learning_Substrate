/**
 * Put this file inside `src/examples/` folder
 */
import { ApiPromise, WsProvider } from "@polkadot/api";

export async function main(rpc_api_url: string) {
    const rpcApiUrl = rpc_api_url;
    // Construct
    const wsProvider = new WsProvider(rpcApiUrl);
    const api = await ApiPromise.create({
        provider: wsProvider,
    });
    await api.isReady;

    // Retrieve the chain & node information information via rpc calls
    const [chain, nodeName, nodeVersion] = await Promise.all([
        api.rpc.system.chain(),
        api.rpc.system.name(),
        api.rpc.system.version(),
    ]);

    console.log(
        `You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`
    );
}
