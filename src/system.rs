use std::collections::BTreeMap;


// Define cutstom types to be more readable
type AccountID = String;
type BlockNumber = u32;
type Nonce = u32;
#[derive(Debug)]
pub struct Pallet {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountID, Nonce>
}

impl Pallet {
    pub fn new() -> Self {
        Self { 
            block_number: 0, 
            nonce: BTreeMap::new() 
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        // crashed if overflow for purpose
        self.block_number = self.block_number.checked_add(1).unwrap();
    } 

    pub fn inc_nonce(&mut self, who: &AccountID) {
        let nonce = self.nonce.get(who).unwrap_or(&0);
        self.nonce.insert(who.clone(), nonce + 1);
    }

    pub fn get_nonce(&mut self, who: &AccountID) -> u32 {
        *self.nonce.get(who).unwrap_or(&0)
    }

}


mod test {
    #[test]
    fn init_system() {
       let system = super::Pallet::new();
       assert_eq!(system.block_number(), 0)
    }

    #[test]
    fn inc_block_number() {
        let mut system = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number(), 1)

    }
    #[test]
    fn inc_nonce() {
        let alice = String::from("alice");

        let mut system = super::Pallet::new();
        system.inc_nonce(&alice.clone());
        assert_eq!(system.get_nonce(&alice), 1);

    }
}