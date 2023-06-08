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