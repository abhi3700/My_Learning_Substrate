//! The way to have external (dispatchable) calls which are the ones that can be called from outside the runtime
//! Normally returns: `PostDispatchInfo` (weight, pays_fee) that has transaction related info
//! But, if there is a function that returns `PostDispatchInfo` then it will return `DispatchResultWithPostInfo`
//! which overrides the default `DispatchResult` that is returned by the dispatchable functions

impl <T> Module <T> where T: Trait {
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[weight = 0]
    pub fn dispatchable_call(origin) -> DispatchResult {
        // Check that the extrinsic was signed and get the signer.
        // This function will return an error if the extrinsic is not signed.
        // https://substrate.dev/docs/en/knowledgebase/runtime/origin
        let who = ensure_signed(origin)?;

        // Update storage.
        <Something<T>>::put(who);

        // Return a successful DispatchResult
        Ok(())
    }
}