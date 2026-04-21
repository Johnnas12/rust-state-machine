use std::{collections::BTreeMap, ops::AddAssign};

use num::traits::{Zero, One, CheckedAdd, CheckedSub};


// Define cutstom types to be more readable
type AccountID = String;
type BlockNumber = u32;
type Nonce = u32;


#[derive(Debug)]
pub struct Pallet <AccountID, BlockNumber, Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountID, Nonce>
}

impl <AccountID, BlockNumber, Nonce> Pallet <AccountID, BlockNumber, Nonce> 
where  
    AccountID: Ord + Clone,
    BlockNumber: Ord + One +  Clone + Copy + Zero + AddAssign,
    Nonce: Ord + Clone + Copy + Zero + One
{
    pub fn new() -> Self {
        Self { 
            block_number: BlockNumber::zero(), 
            nonce: BTreeMap::new() 
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        // crashed if overflow for purpose
        self.block_number += BlockNumber::one();
    } 

    pub fn inc_nonce(&mut self, who: &AccountID) {
        let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
        self.nonce.insert(who.clone(), nonce + Nonce::one());
    }

    pub fn get_nonce(&mut self, who: &AccountID) -> Nonce {
        *self.nonce.get(who).unwrap_or(&Nonce::zero())
    }

}


mod test {
    use crate::system::Pallet;

    #[test]
    fn init_system() {
       let system: Pallet<String, u32 , u32> = super::Pallet::new();
       assert_eq!(system.block_number(), 0)
    }

    #[test]
    fn inc_block_number() {
        let mut system:  Pallet<String, u32 , u32> = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number(), 1)

    }
    #[test]
    fn inc_nonce() {
        let alice = String::from("alice");

        let mut system:  Pallet<String, u32 , u32> = super::Pallet::new();
        system.inc_nonce(&alice.clone());
        assert_eq!(system.get_nonce(&alice), 1);

    }
}