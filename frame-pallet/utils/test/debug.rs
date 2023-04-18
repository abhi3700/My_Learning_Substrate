#[pallet]
mod pallet {
    use frame_support::log;

    // use inside dispatchable (function).
    #[pallet::call]
    fn say_hello(origin: OriginFor<T>) -> DispatchResult {
        let who: <T as Config>::AccountId = ensure_signed(origin)?;

        // only seen on console during `$ RUST_LOG=runtime=debug ./target/release/node-template --dev`
        // to show as debug at debug level
        // Source: https://docs.rs/log/0.4.14/log/macro.debug.html
        log::debug!("called by {:?}", who);
    }
}
