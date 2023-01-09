/*
    Constructor:
    - add at least 1 constructors - `new` [Recommended], `default`
*/

#[ink(constructor)]
pub fn new(total_supply: Balance) -> Self {
    let mut balances = Mapping::default();
    let caller = Self::env().caller();
    balances.insert(caller, &total_supply);
    Self::env().emit_event(Transfer {
        from: None,
        to: Some(caller),
        value: total_supply,
    });
    Self {
        total_supply,
        balances,
        allowances: Default::default(),
    }
}

// Although not implemented in PSP22 code
#[ink(constructor)]
pub fn default() -> Self {
    Self::new()
}
