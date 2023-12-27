//! Keep the type of variable as hash

let content: Vec<u8> = b"hello world";   // in bytes format i.e. Vec<u8>
let content_hash: [u8; 32] = sp_io::hashing::blake2_256(&content); // in [u8; 32] format
let content_hash: sp_core::H256 = sp_io::hashing::blake2_256(&content).into(); // in H256 format
