//! This has to be implemented in all pallet inside the `mod pallet` in `src/lib.rs`
//! even if it is empty (like in `hello-substrate` pallet)
//! NOTE: in case of no event type inside `Config` trait, there won't be any any
//! event emission possible as you won't be able to define Events

pub trait Config: frame_system::Config {
    // type RuntimeEvent: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}
