# Substrate SDK

This contains the Substrate SDK, a collection of tools and libraries to get the task done.

> For instance, generating a random wallet, connecting to a node, sending a transaction, etc. All these are possible with the help of Substrate SDK and its libraries.

## Coding

Some general guidelines for coding.

### 1. Wallet

<details><summary><b>Generate random wallet</b></summary>

[Code](https://github.com/abhi3700/subspace-dev-prerequisites/blob/main/subspace-sdk-playground/examples/keypair_frm_secretph.rs)

</details>

<details><summary><b>Generate wallet from given seed phrase</b></summary>

[Code](https://github.com/abhi3700/subspace-dev-prerequisites/blob/main/subspace-sdk-playground/examples/keypair_frm_secretph_known.rs)

</details>

<details><summary><b>Generate random seed phrase</b></summary>

[Code](https://github.com/abhi3700/subspace-dev-prerequisites/blob/main/subspace-sdk-playground/examples/secret_ph.rs)

</details>

<details><summary><b>Convert SS58 to your Address format</b></summary>

[Code](https://github.com/abhi3700/subspace-dev-prerequisites/blob/main/subspace-sdk-playground/examples/addr_substrate_to_subspace.rs)

In Substrate and many Substrate-based chains, public keys are used as addresses. These public keys are usually of a specific type, like `sp_core::ed25519::Public`, `sp_core::sr25519::Public`, or `sp_core::ecdsa::Public`, all of which implement the `sp_core::Public` trait.

The solution is to determine the correct type of public key that your addresses use. If you're not sure, `sp_core::sr25519::Public` is commonly used in many Substrate-based chains.

Here's how you might modify your code to use `sp_core::sr25519::Public`:

```rust
use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
use sp_core::sr25519::Public;

fn convert_address(address: &str, new_format: Ss58AddressFormat) -> Result<String, &'static str> {
    let public_key = match Public::from_ss58check_with_version(address) {
        Ok((public_key, _)) => public_key,
        Err(_) => return Err("Invalid address"),
    };

    Ok(public_key.to_ss58check_with_version(new_format))
}

fn main() {
    let address = "5DepcPH2mXTLeLubiF5kXdbtgdFKc53mt6J6Ne8ethqpVem4";
    let new_format = Ss58AddressFormat::custom(2254);
    match convert_address(address, new_format) {
        Ok(new_address) => println!("New address: {}", new_address),
        Err(err) => println!("Error: {}", err),
    }
}
```

If your addresses use a different public key type (like `sp_core::ed25519::Public` or `sp_core::ecdsa::Public`), you'll need to adjust the code accordingly.

</details>

### 2. Storage

### 3. Events

### 4. RPC

### 5. Transactions

### 6. Off-chain workers

### 7. Runtime

### 8. Pallets

### 9. Substrate Node
