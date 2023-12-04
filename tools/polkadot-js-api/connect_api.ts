/**
 * Put this file inside `src/examples/` folder
 */
import { createSubstrateApiInstance } from "./create_api";

async function connectApi(rpc_api_url: string) {
  // Create the API and wait until ready
  const api = await createSubstrateApiInstance(rpc_api_url);

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
