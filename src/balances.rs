use std::collections::BTreeMap;

use num::traits::{CheckedAdd, CheckedSub, Zero};

// first lets define out trait
pub trait Config : crate::system::Config{
    type Balance: Zero + CheckedAdd + CheckedSub + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountID, T::Balance>
}

impl <T: Config> Pallet<T> 
    {
    pub fn new() -> Self {
        Self {
            balances : BTreeMap::new()
        }
    }

    pub fn set_balance(&mut self, who: &T::AccountID, amount: T::Balance ){
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&mut self, who: &T::AccountID) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

pub fn transfer(&mut self, caller: T::AccountID, to: T::AccountID, amount: T::Balance) -> Result<(), &'static str> {
    // 1. Get current balances
    let caller_balance = self.balance(&caller);
    let to_balance = self.balance(&to);

    let new_caller_balance = caller_balance.checked_sub(&amount)
        .ok_or("Insufficient Balance")?;
    
    let new_to_balance = to_balance.checked_add(&amount)
        .ok_or("Overflow")?;

    // 3. Update state
    self.set_balance(&caller, new_caller_balance);
    self.set_balance(&to, new_to_balance);

    Ok(())
}
}


mod tests{
    pub struct TestConfig;

    impl super::Config for TestConfig {
        type Balance = u128;
    }

    impl crate::system::Config for TestConfig {
        type AccountID = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

#[test]
pub fn init_balance() {
    let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

    assert_eq!(balances.balance(&"Alice".to_string()), 0);
    balances.set_balance(&"Alice".to_string(), 100);
    assert_eq!(balances.balance(&"Alice".to_string()), 100);
    assert_eq!(balances.balance(&"Bob".to_string()), 0)
}


#[test]
pub fn transfer_balance() {
    let alice = "Alice".to_string();
    let bob = "Bob".to_string();

    let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

    balances.set_balance(&alice.clone(), 100);
    let _ = balances.transfer(alice.clone(), bob.clone(), 90);

    assert_eq!(balances.balance(&bob), 90);
    assert_eq!(balances.balance(&alice), 10);
}

#[test]
pub fn test_insufficient_balance_case() {
    let alice = "Alice".to_string();
    let bob = "Bob".to_string();

    let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

    balances.set_balance(&alice, 100);
    let result = balances.transfer(alice.clone(), bob.clone(), 110);

    assert_eq!(result, Err("Insufficient Balance"));
    assert_eq!(balances.balance(&alice), 100);
    assert_eq!(balances.balance(&bob), 0);
}

#[test]
pub fn test_overflow_case() {
    let alice = "Alice".to_string();
    let bob ="bob".to_string();

    let mut balances: super::Pallet<TestConfig> = super::Pallet::new();

    balances.set_balance(&bob, u128::MAX);
    balances.set_balance(&alice, 100);

    let result = balances.transfer(alice.clone(), bob.clone(), 1);
    assert_eq!(result, Err("Overflow"));
    assert_eq!(balances.balance(&bob), u128::MAX);
    assert_eq!(balances.balance(&alice), 100);
}

}