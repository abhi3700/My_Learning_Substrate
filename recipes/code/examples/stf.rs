//! Create a State Transition Function (STF) which creates a new state using the current block & old state
//!
//! Here, coin transfer as activity is used as current block data.
//! And the previous data is `10`. The code logic for transition function
//! has been assumed with formula as: `old_state + length(current_block.data)`
//!
//! Doc: https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/blockchain_state_machines/index.html

struct State {
    value: u32,
}

struct Block {
    data: String,
}

impl Block {
    fn new(data: String) -> Self {
        Self { data }
    }
}

/// A STF returning the old state
fn stf(previous_state: State, current_block: Block) -> State {
    let new_state = previous_state.value + current_block.data.len() as u32;
    return State { value: new_state };
}

fn main() {
    // previous state
    let previous_state = State { value: 10 };
    println!("Old state: {:?}", previous_state.value);

    // current block
    let current_block = Block::new("transfer coin from Alice to Bob".to_string());
    let new_state = stf(previous_state, current_block);

    println!("New state: {:?}", new_state.value); // 41
}
