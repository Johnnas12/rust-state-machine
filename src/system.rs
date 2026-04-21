use std::{collections::BTreeMap, ops::AddAssign};

use num::traits::{Zero, One, CheckedAdd, CheckedSub};

pub trait  Config {
    type AccountID: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + Copy;
}


// Define cutstom types to be more readable
type AccountID = String;
type BlockNumber = u32;
type Nonce = u32;


#[derive(Debug)]
pub struct Pallet <T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountID, T::Nonce>
}

impl <T: Config> Pallet <T> 
{
    pub fn new() -> Self {
        Self { 
            block_number: T::BlockNumber::zero(), 
            nonce: BTreeMap::new() 
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        // crashed if overflow for purpose
        self.block_number += T::BlockNumber::one();
    } 

    pub fn inc_nonce(&mut self, who: &T::AccountID) {
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        self.nonce.insert(who.clone(), nonce + T::Nonce::one());
    }

    pub fn get_nonce(&mut self, who: &T::AccountID) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }

}


mod test {
    struct  TestConfig;

    impl super::Config for TestConfig {
        type AccountID = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
       let system: super::Pallet<TestConfig> = super::Pallet::new();
       assert_eq!(system.block_number(), 0)
    }

    #[test]
    fn inc_block_number() {
        let mut system:  super::Pallet<TestConfig> = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number(), 1)

    }
    #[test]
    fn inc_nonce() {
        let alice = String::from("alice");

        let mut system:  super::Pallet<TestConfig> = super::Pallet::new();
        system.inc_nonce(&alice.clone());
        assert_eq!(system.get_nonce(&alice), 1);

    }
}