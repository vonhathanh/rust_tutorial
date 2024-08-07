use std::{
    collections::{HashMap, HashSet},
    vec,
};

use alloy_primitives::{Address, Bytes};
use ruint::aliases::{B256, U256};

pub struct WorldState {
    // A mapping between addresses (160-bit identifiers) and account state (a data structure serialized as RLP)
    // Though not stored on the blockchain, its assumed that the implementation will maintain this mapping using a
    // modified Merkle Patricia Tree.
    // The trie requires a simple key value database backend that maintains a mapping of bytes array to bytes array.
    // Benefits: the root node of the trie is cryptographically dependent on all internal data so it
    // can be used as a secure identity for the entire system state
    // Secondly, it allows any previous state (whose root hash is known) to be recalled by simply
    // altering the root hash accordingly. Since we store all such root hashes in the blockchain, we are able to
    // trivially revert to old states.
    pub trie: HashMap<Address, AccountState>,
}

impl WorldState {
    pub fn new() -> Self {
        WorldState {
            trie: HashMap::new(),
        }
    }
}

pub struct AccountState {
    // Number of transactions sent from this address or
    // in the case of contract-account, the number of contract-creation made by this account
    pub nonce: U256,
    // Number of Wei owned by this address
    pub balance: U256,
    // A 256 bit hash of the root node of the trie that encode the storage content of an account
    // (a mapping between 256 bit integer values)
    // encoded into the trie as: keccak256(all keys) -> RLP encoded of all values
    // Note: storage_root is not a "physical" member of the AccountState and does not contribute to it serialization
    pub storage_root: B256,
    // The hash of the EVM byte codes of this address
    pub code_hash: B256,
    // code: if None, do nothing otherwise load the code from storage if it need to execute calls
    pub code: Option<Bytes>,
}

pub struct SubState {
    // Accrued information that is acted upon immediately following the transaction
    // A_s: set of accounts that will be discarded following the transaction completion
    pub self_destruct_set: HashSet<Address>,
    // A_l: the log series: series of archived and indexable 'checkpoints' in VM code execution
    // that allow contract-calls to be easily tracked by onlooker external to the Ethereum world
    pub logs: Vec<Bytes>,
    // A_t: set of touched accounts, the empty ones are deleted after transaction end
    pub touched_accounts: HashSet<Address>,
    // A_r: refund balance, increased through using the SSTORE instruction to reset contract storage
    pub refund: U256,
    // EIP-2929, A_a: set of accessed account addresses, A_k, set of accessed storage keys
    // A_k = (address, 32-byte storage slot) why 32-byte? Because 32-byte = 256 bit
    // Note: In the Yellow Paper, A_a is initialized as Pi, set of precompiled addresses, we'll ignore them for now
    // since we don't even know what the heck they are
    pub accessed_accounts: HashSet<Address>,
    pub accessed_storage: HashMap<Address, Bytes>,
}

impl SubState {
    pub fn new() -> Self {
        SubState {
            self_destruct_set: HashSet::new(),
            logs: vec![],
            touched_accounts: HashSet::new(),
            refund: U256::ZERO,
            accessed_accounts: HashSet::new(),
            accessed_storage: HashMap::new(),
        }
    }
}

// Machine state (µ)
// we define it recursively with a function X. This uses an iterator function O (which defines the result of a
// single cycle of the state machine) together with functions Z which determines if the present state is an exceptional
// halting state of the machine and H, specifying the output data of the instruction if and only if the present state is a
// normal halting state of the machine
// Note: emmpty sequence ([]) is different with empty set (())
// empty set -> continue, empty sequence -> halt
pub struct EVMState {
    // gas available
    pub gas: usize,
    // program counter
    pub pc: usize,
    // memory contents: a series of zeroes of size 2^256
    // it's a series, so we use Vec, but no computer can allocate Vec size of 2^256 (no program ever need that fucking memory)
    // so I think we'll be fine
    pub m: Vec<u8>,
    // active number of words in memory
    pub i: usize,
    // stack contents
    pub s: Vec<B256>,
    // returndata buffer
    pub o: Bytes,
}

impl EVMState {
    pub fn new() -> Self {
        EVMState {
            gas: 0,
            pc: 0,
            m: vec![],
            i: 0,
            s: vec![],
            o: Bytes::new(),
        }
    }
}
