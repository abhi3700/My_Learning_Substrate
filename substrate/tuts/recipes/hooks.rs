//! Define hooks for pallet.
//! Hooks are functions that are called by the runtime on specific conditions for a block status.
//! There are different functions/hooks inside a hook trait. And we need to implement them for the pallet, if needed.
//! Otherwise, it can be left empty.
//! https://paritytech.github.io/substrate/master/frame_support/attr.pallet.html#hooks-pallethooks-optional

mod hello_substrate {
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
}
