# Substrate Node

This document focus on everything related to the substrate node development using

- Data indexing tools like OCW,
- DevOps tools like Prometheus, Docker, etc.

## Data Indexing

Things like OCW, Data indexing are laid out comprehensively with rust code examples. Although this document has been created exclusively for Substrate, but the theory is valid for all blockchains in general.

### Blockchain Node

In case of blockchain node, the indexing is supposed to happen on per block basis. Now, in

### Blockchain DApp

The data indexing is also valid for DApps in particular. Suppose there is a DApp & you want to index the data on every transaction/operation/activity basis (may be not every block). Then we need to monitor a common event getting fired which would be present into all txns sent from the DApp.

There are 3 ways to do this:

1. Third-party service: Using Moralis, speedynode, we can look for those events of contracts.
2. Build your own: On a per block basis, we have to look for the the smart contract’s events.
3. When the activity is sent from DApp, we take the data & send via API endpoint. And then inside the handler function we spawn a separate thread for the blockchain transaction validation & then receiving the transaction_id we index/store the data accordingly to have the best performance.

3 > 2 > 1 (best performance)

## References
