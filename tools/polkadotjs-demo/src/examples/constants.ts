/**
 * Put this file inside `src/examples/` folder
 */

// export const POLKADOT_API_URL = "wss://rpc.polkadot.io";
// export const POLKADOT_API_URL = "wss://polkadot.api.onfinality.io/public-ws";

// take API key from here: https://app.onfinality.io/
const ON_FINALITY_API_KEY = "5d0b0b5c-0b9e-4b7e-9b0e-6b7bdc331b7e";
export const POLKADOT_API_URL = `wss://polkadot.api.onfinality.io/ws?apikey=${ON_FINALITY_API_KEY}`;
