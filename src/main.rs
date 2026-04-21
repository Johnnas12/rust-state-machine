use crate::{
    support::Dispatch,
    types::{AccountID, Balance, BlockNumber, Nonce},
};

mod balances;
mod support;
mod system;

mod types {
    use crate::{RuntimeCall, support};

    pub type Balance = u128;
    pub type AccountID = String;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = support::Extrinsic<AccountID, RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {
    BalanceTransfer{to: types::AccountID, amount: types::Balance},
}

impl system::Config for Runtime {
    type AccountID = AccountID;
    type BlockNumber = BlockNumber;
    type Nonce = Nonce;
}

impl balances::Config for Runtime {
    type Balance = Balance;
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }

    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();

        if (self.system.block_number() != block.header.block_number) {
            return Err("Block number Mistmatch");
        }

        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);
            let _ = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error \n\t Block Number: {} \n\t  Extrinsic Number: {}\n\t Error {}",
                    block.header.block_number, i, e
                )
            });
        }
        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountID;
    type Call = RuntimeCall;

    fn dispatch(&mut self, caller: Self::Caller, runtime_call: Self::Call) -> support::DispatchResult {

        match runtime_call {
            RuntimeCall::BalanceTransfer { to, amount } => {
                self.balances.transfer(caller, to, amount)?;
            }
        }
        Ok(())
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
    let _ = runtime
        .balances
        .transfer(alice.clone(), bob.clone(), 30)
        .map_err(|e| println!("Error: {:?}", e));

    runtime.system.inc_nonce(&alice);
    let _ = runtime
        .balances
        .transfer(alice.clone(), charlie.clone(), 20)
        .map_err(|e| println!("Error: {:?}", e));

    println!("{:#?}", runtime)
}