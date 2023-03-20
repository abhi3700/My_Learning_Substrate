//! This has to be implemented in all pallet inside the `mod pallet` in `src/lib.rs` 
//! even if it is empty (like in `hello-substrate` pallet)

pub trait Config:: frame_system::Config {
    // type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}