use crate::types::{AccountID, Balance, BlockNumber, Nonce};

mod balances;
mod system;

mod types {
    pub type Balance = u128;
    pub type AccountID = String;
    pub type BlockNumber = u32;
    pub type Nonce = u32;

}

impl system::Config for Runtime  {
    type AccountID = AccountID;
    type BlockNumber = BlockNumber;
    type Nonce = Nonce;
}

impl balances::Config for Runtime {
    type Balance = Balance;
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet <Runtime>,
    balances: balances::Pallet<Runtime>,

}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    // simulate the transactions and all the other things

    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    // initialize alices balance
    runtime.balances.set_balance(&alice, 100);

    runtime.system.inc_block_number();

    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&alice);
    let _ = runtime.balances.transfer(alice.clone(), bob.clone(), 30)
            .map_err(|e| println!("Error: {:?}", e));
    

    runtime.system.inc_nonce(&alice);
    let _ = runtime.balances.transfer(alice.clone(), charlie.clone(), 20)
        .map_err(|e| println!("Error: {:?}", e));

    println!("{:#?}", runtime)



}

