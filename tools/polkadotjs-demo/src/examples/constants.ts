/**
 * Put this file inside `src/examples/` folder
 */

// export const POLKADOT_API_URL = "wss://rpc.polkadot.io";
// export const POLKADOT_API_URL = "wss://polkadot.api.onfinality.io/public-ws";

// take API key from here: https://app.onfinality.io/
import { config } from "dotenv";

// load env vars
const myEnv = config();

// Check if the .env file is loaded or specific variables are set
if (myEnv.error) {
    throw new Error("Failed to load the .env file.");
}

const ON_FINALITY_API_KEY = process.env.ON_FINALITY_API_KEY;
if (!ON_FINALITY_API_KEY) {
    throw new Error("Please set your private key in a .env file");
}

export const POLKADOT_RPC_API_URL = `wss://polkadot.api.onfinality.io/ws?apikey=${ON_FINALITY_API_KEY}`;
export const SUBSPACE_RPC_API_URL = `ws://`; // TODO: add Subspace API URL
