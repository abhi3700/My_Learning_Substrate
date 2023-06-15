//! Source: https://substrate.recipes/runtime-printing.html#printing-from-the-runtime
//! Here, there is a `sp_runtime::print` function that can be used to print to the terminal
//! There is another called `frame_support::debug::info` that can be used to print to the terminal along with
//! the caller's account id

use frame_support::log;
use frame_support::sp_runtime::print;

print("Hello, world!");
log::info("called by {:?}", who);
log::debug!("called by {:?}", who); // ❌ didn't work for me

// All the logs can be filtered using `-lmytarget=debug` flag.

// And that's how you print from the runtime in native

// But, if you want to print from the runtime in WASM, you need to use the `debug::RuntimeLogger::init()`;

// One usage in OCW codebase:
match res {
    Ok(()) => log::info!("[{:?}] Submitted price of {} cents", acc.id, price),
    Err(e) => log::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
}

// this is run in non-optimized build
debug_assert!(ok, "`seconds` is below static limit; `try_insert` should succeed; qed");

// with TARGET refined
log::warn!(
    target: LOG_TARGET,
    "account with a non-zero reserve balance has no provider refs, account_id: '{:?}'.",
    who
);
