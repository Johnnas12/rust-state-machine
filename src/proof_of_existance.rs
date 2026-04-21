use core::fmt::Debug;
use std::collections::BTreeMap;
use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
	/* TODO: Add a field `claims` which is a `BTreeMap` fom `T::Content` to `T::AccountId`. */
    claims: BTreeMap<T::Content, T::AccountID>
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
        Self {
            claims: BTreeMap::new()
        }
	}

    pub fn get_claims(&self, claim: &T::Content) -> Option<&T::AccountID> {
        self.claims.get(claim)
    }

    pub fn create_claim(&mut self, caller: T::AccountID, claim: T::Content) -> DispatchResult {
        if self.claims.contains_key(&claim){
            return Err(&"This content is already claimed");
        }

        self.claims.insert(claim, caller);
        Ok(())
    }

    pub fn revoke_claim(&mut self, caller: T::AccountID, claim: T::Content) -> DispatchResult {
        let claim_owner  = self.get_claims(&claim).ok_or("Claim owner doesnt exist")?;
        if claim_owner != &caller {
            return  Err("Caller is not the owner of the claim");
        }
        self.claims.remove(&claim);
		Ok(())
	}


}

pub enum Call<T: Config> {
    CreateClaim{claim: T::Content},
    RevokeClaim {claim: T::Content}
}


impl <T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountID;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::CreateClaim { claim } => {  self.create_claim(caller, claim) },
            Call::RevokeClaim { claim } => { self.revoke_claim(caller, claim)}
        }
    }
}


#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountID = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {

        let mut poe = super::Pallet::<TestConfig>::new();
        let _ = poe.create_claim("alice", "my_document");

        assert_eq!(poe.get_claims(&"my_document"), Some(&"alice"));

        let res = poe.revoke_claim("bob", "my_document");
        assert_eq!(res, Err("Caller is not the owner of the claim"));

        let res2 = poe.create_claim("bob", "my_document");
        assert_eq!(res2, Err("This content is already claimed"));

        let res3 = poe.revoke_claim("alice", "non existant");
        assert_eq!(res3, Err("Claim owner doesnt exist"));

        let res = poe.revoke_claim("alice", "my_document");
        assert_eq!(res, Ok(()));
        assert_eq!(poe.get_claims(&"my_document"), None);
	}
}