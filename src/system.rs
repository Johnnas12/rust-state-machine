use std::collections::BTreeMap;


pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>
}

impl Pallet {
    pub fn new() -> Self {
        Self { 
            block_number: 0, 
            nonce: BTreeMap::new() 
        }
    }

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

}


mod test {
    #[test]
    fn init_system() {
       let system = super::Pallet::new();
       assert_eq!(system.block_number(), 0) 
    }
}