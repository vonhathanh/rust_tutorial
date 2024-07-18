use alloy_primitives::{hex::FromHex, Address, B256, U256};
use state::{EVMState, SubState, WorldState};
use transaction::Transaction;

mod block;
mod evm;
mod functions;
mod operations;
mod state;
mod transaction;

fn main() {
    let mut world_state = WorldState::new();
    let mut evm_state = EVMState::new();
    let mut accrued_substate = SubState::new();
    let tx = Transaction::new(
        0,
        U256::from(1),
        100000,
        Some(Address::from_hex("0x6148ce093dcbd629cfbc4203c18210567d186c66").unwrap()),
        U256::ZERO,
        B256::ZERO,
        B256::ZERO,
        None,
        Some(1),
        Some(1),
        None,
        None,
        None,
        Some(U256::from(1000)),
        None,
        None
    );
}
