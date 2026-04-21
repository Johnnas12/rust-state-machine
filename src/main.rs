use crate::{
    balances::Call, support::Dispatch, types::{AccountID, Balance, BlockNumber, Nonce}
};

mod balances;
mod support;
mod system;
mod proof_of_existance;

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
    Balances(balances::Call<Runtime>)
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
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
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

    runtime.balances.set_balance(&alice, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer  { to: bob.clone(), amount: 30 }), 
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie.clone(), amount: 9 })
            }
        ]
    };

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer { to: bob.clone(), amount: 15 }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie.clone(), amount: 20}),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer { to: alice.clone(), amount: 10})
            }
        ]
    };

    runtime.execute_block(block_1).expect("Wrong Block Execution");    
    runtime.execute_block(block_2).expect("Wrong Block execution");
    println!("{:#?}", runtime)
}