//! Get free, reserved, total balance of a user

impl<T: Config> Pallet<T> {
    fn get_frt_balances(
        caller: &T::AccountId,
    ) -> Result<(BalanceOf<T>, BalanceOf<T>, BalanceOf<T>), DispatchError> {
        let f = T::MyCurrency::free_balance(&caller);
        let r = T::MyCurrency::reserved_balance(&caller);
        let t = T::MyCurrency::total_balance(&caller);

        Ok((f, r, t))
    }
}
