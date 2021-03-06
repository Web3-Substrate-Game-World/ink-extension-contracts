#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;
use ink_prelude::{
    // boxed::Box,
    // collections::BTreeMap,
    // string::String,
    // vec,
    vec::Vec,
};

type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
type TokenId = u64;
type TaoId = u64;
// type TokenBalance = u128;

/// Define the operations to interact with the substrate runtime
#[ink::chain_extension]
pub trait Erc1155 {
    type ErrorCode = ExtensionReadErr;

    #[ink(extension = 1001, returns_result = false)]
    fn fetch_random() -> [u8; 32];

    #[ink(extension = 1002, returns_result = false)]
    fn create_tao(who: AccountId, data: Vec<u8>) -> TokenId;

    #[ink(extension = 1003, returns_result = false)]
    fn create_token(who: AccountId, tao_id: TaoId);

    #[ink(extension = 1004, returns_result = false)]
    fn set_approval_for_all(owner: AccountId, operator: AccountId, approved: bool);

    #[ink(extension = 1005, returns_result = false)]
    fn mint(to: AccountId, tao_id: TaoId, token_id: TokenId, amount: Balance);

    #[ink(extension = 1006, returns_result = false)]
    fn batch_mint(to: AccountId, tao_id: TaoId, token_ids : Vec<TokenId>, amounts: Vec<Balance>);

    #[ink(extension = 1007, returns_result = false)]
    fn burn(from: AccountId, tao_id: TaoId, token_id: TokenId, amount: Balance);

    #[ink(extension = 1008, returns_result = false)]
    fn batch_burn(from: AccountId, tao_id: Vec<TokenId>, token_ids: Vec<TokenId>, amounts: Vec<Balance>);

    #[ink(extension = 1009, returns_result = false)]
    fn transfer_from(from: AccountId, to: AccountId, tao_id: TaoId, token_id: TokenId, amount: Balance);

    #[ink(extension = 1010, returns_result = false)]
    fn batch_transfer_from(from: AccountId, to: AccountId, tao_id: TaoId, token_ids: Vec<TokenId>, amounts: Vec<Balance>);

    #[ink(extension = 1011, returns_result = false)]
    fn approved_or_owner(who: AccountId, account: AccountId) -> bool;

    #[ink(extension = 1012, returns_result = false)]
    fn is_approved_for_all(owner: AccountId, operator: AccountId) -> bool;

    #[ink(extension = 1013, returns_result = false)]
    fn balance_of(owner: AccountId, tao_id: TaoId, token_id: TokenId)  -> Balance;

    #[ink(extension = 1014, returns_result  = false)]
    fn balance_of_batch(owners: Vec<AccountId>, tao_id: TaoId, token_ids: Vec<TokenId>) -> Vec<Balance>;

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ExtensionReadErr {
    FailGetRandomSource,
}

impl ink_env::chain_extension::FromStatusCode for ExtensionReadErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailGetRandomSource),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = Erc1155;
}

#[ink::contract(env = crate::CustomEnvironment)]
mod erc1155_extension {
    use super::ExtensionReadErr;
    use crate::{/*vec,*/ Vec, TokenId, TaoId /*, TokenBalance*/};

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Erc1155Extension {
        /// Stores a single `bool` value on the storage.
        value: [u8; 32],
    }

    #[ink(event)]
    pub struct RandomUpdated {
        #[ink(topic)]
        new: [u8; 32],
    }

    impl Erc1155Extension {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: Default::default() }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        /// update the value from runtime random source
        #[ink(message)]
        pub fn update_random(&mut self) -> Result<(), ExtensionReadErr> {
            // Get the on-chain random seed
            let new_random = self.env().extension().fetch_random()?;
            self.value = new_random;
            // emit the RandomUpdated event when the random seed
            // is successfully fetched.
            self.env().emit_event(RandomUpdated { new: new_random });
            Ok(())
        }

        /// Simply returns the current value.
        #[ink(message)]
        pub fn get_random(&self) -> [u8; 32] {
            self.value
        }

       #[ink(message)]
        pub fn create_tao(&mut self, who: AccountId, data: Vec<u8>) -> Result<TokenId, ExtensionReadErr> {
           let ret = self.env().extension().create_tao(who, data)?;

           Ok(ret)
        }

        #[ink(message)]
        pub fn create_token(&mut self, who: AccountId, tao_id: TaoId) -> Result<(), ExtensionReadErr> {
            self.env().extension().create_token(who, tao_id)?;

            Ok(())
        }



        #[ink(message)]
        pub fn set_approval_for_all(&mut self, owner: AccountId, operator: AccountId, approved: bool) -> Result<(), ExtensionReadErr> {
            self.env().extension().set_approval_for_all(owner, operator, approved)?;

            Ok(())
        }

        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, tao_id: TaoId, token_id: TokenId, amount: Balance) -> Result<(), ExtensionReadErr> {
            self.env().extension().mint(to, tao_id, token_id, amount)?;

            Ok(())
        }

        #[ink(message)]
        pub fn batch_mint(&mut self, to: AccountId, tao_id: TaoId, token_ids : Vec<TokenId>, amounts: Vec<Balance>) -> Result<(), ExtensionReadErr> {
            self.env().extension().batch_mint(to, tao_id, token_ids, amounts)?;

            Ok(())
        }

        #[ink(message)]
        pub fn burn(&mut self, from: AccountId, tao_id: TaoId, token_id: TokenId, amount: Balance) -> Result<(), ExtensionReadErr> {
            self.env().extension().burn(from, tao_id, token_id, amount)?;

            Ok(())
        }

        #[ink(message)]
        pub fn batch_burn(&mut self, from: AccountId, tao_id: Vec<TokenId>, token_ids: Vec<TokenId>, amounts: Vec<Balance>) -> Result<(), ExtensionReadErr> {
            self.env().extension().batch_burn(from, tao_id, token_ids, amounts)?;

            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, tao_id: TaoId, token_id: TokenId, amount: Balance) -> Result<(), ExtensionReadErr> {

            self.env().extension().transfer_from(from, to, tao_id, token_id, amount)?;

            Ok(())
        }

        #[ink(message)]
        pub fn batch_transfer_from(&mut self, from: AccountId, to: AccountId, tao_id: TaoId, token_ids: Vec<TokenId>, amounts: Vec<Balance>) -> Result<(), ExtensionReadErr> {

            self.env().extension().batch_transfer_from(from, to, tao_id,token_ids,  amounts)?;

            Ok(())
        }

        #[ink(message)]
        pub  fn approved_or_owner(&self, who: AccountId, account: AccountId) -> Result<bool, ExtensionReadErr>{
            let ret = self.env().extension().approved_or_owner(who, account)?;

            Ok(ret)
        }



        #[ink(message)]
        pub fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> Result<bool, ExtensionReadErr> {
            let ret = self.env().extension().is_approved_for_all(owner, operator)?;

            Ok(ret)
        }


        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId, tao_id: TaoId, token_id: TokenId) -> Result<Balance, ExtensionReadErr> {
            let balance = self.env().extension().balance_of(owner, tao_id, token_id)?;

            Ok(balance)
        }


        #[ink(message)]
        pub fn balance_of_batch(&self, owners: Vec<AccountId>, tao_id: TaoId, token_ids: Vec<TokenId>) -> Result<Vec<Balance>, ExtensionReadErr> {
            let balance = self.env().extension().balance_of_batch(owners, tao_id, token_ids)?;

            Ok(balance)
        }

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[test]
        fn default_works() {
            let erc1155_extension = Erc1155Extension::default();
            assert_eq!(erc1155_extension.get(), false);
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
            let mut erc1155_extension = Erc1155Extension::new(false);
            assert_eq!(erc1155_extension.get(), false);
            erc1155_extension.flip();
            assert_eq!(erc1155_extension.get(), true);
        }
    }
}
