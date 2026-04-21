use std::collections::BTreeMap;

use num::traits::{CheckedAdd, CheckedSub, Zero};

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
    balances: BTreeMap<AccountId, Balance>
}

impl <AccountId, Balance> Pallet<AccountId, Balance> 
    where  
        AccountId: Ord + Clone, 
        Balance: Zero + CheckedSub + CheckedAdd + Copy,
    {
    pub fn new() -> Self {
        Self {
            balances : BTreeMap::new()
        }
    }

    pub fn set_balance(&mut self, who: &AccountId, amount: Balance ){
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&mut self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

pub fn transfer(&mut self, caller: AccountId, to: AccountId, amount: Balance) -> Result<(), &'static str> {
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


#[test]
pub fn init_balance() {
    let mut balances = super::Pallet::new();

    assert_eq!(balances.balance(&"Alice".to_string()), 0);
    balances.set_balance(&"Alice".to_string(), 100);
    assert_eq!(balances.balance(&"Alice".to_string()), 100);
    assert_eq!(balances.balance(&"Bob".to_string()), 0)
}


#[test]
pub fn transfer_balance() {
    let alice = "Alice".to_string();
    let bob = "Bob".to_string();

    let mut balances = super::Pallet::new();

    balances.set_balance(&alice.clone(), 100);
    let _ = balances.transfer(alice.clone(), bob.clone(), 90);

    assert_eq!(balances.balance(&bob), 90);
    assert_eq!(balances.balance(&alice), 10);
}

#[test]
pub fn test_insufficient_balance_case() {
    let alice = "Alice".to_string();
    let bob = "Bob".to_string();

    let mut balances: super::Pallet<String, u128> = super::Pallet::new();

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

    let mut balances = super::Pallet::new();

    balances.set_balance(&bob, u128::MAX);
    balances.set_balance(&alice, 100);

    let result = balances.transfer(alice.clone(), bob.clone(), 1);
    assert_eq!(result, Err("Overflow"));
    assert_eq!(balances.balance(&bob), u128::MAX);
    assert_eq!(balances.balance(&alice), 100);
}

}