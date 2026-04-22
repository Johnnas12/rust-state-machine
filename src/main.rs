use std::iter::Product;

use crate::{
    balances::Call, support::Dispatch, types::{AccountId, Balance, BlockNumber, Nonce}
};

mod balances;
mod support;
mod system;
mod proof_of_existance;

mod types { 
    use crate::{RuntimeCall, support};

    pub type Balance = u128;
    pub type AccountId = String;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = support::Extrinsic<AccountId, RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}

impl system::Config for Runtime {
    type AccountId = AccountId;
    type BlockNumber = BlockNumber;
    type Nonce = Nonce;
}

impl balances::Config for Runtime {
    type Balance = Balance;
}

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
    proof_of_existance: proof_of_existance::Pallet<Runtime>
}

impl proof_of_existance::Config for Runtime {
    type Content = types::Content;
}

fn main() {
    // simulate the transactions and all the other things

    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer  { to: bob.clone(), amount: 30 }), 
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer { to: charlie.clone(), amount: 9 })
            }
        ]
    };

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existance(proof_of_existance::Call::create_claim { claim: "My document"}),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer { to: charlie.clone(), amount: 20}),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existance(proof_of_existance::Call::create_claim { claim: "My document"}),
            }
        ]
    };

    runtime.execute_block(block_1).expect("Wrong Block Execution");    
    runtime.execute_block(block_2).expect("Wrong Block execution");
    println!("{:#?}", runtime)
}