#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
//(env = DefaultEnvironment)
#[ink::contract]
mod erc20 {

    use ink_storage::collections::HashMap as StorageHashMap;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Erc20 {
        
        total_supply:Balance,
        balances:StorageHashMap<AccountId, Balance>,
        allowance:StorageHashMap<(AccountId,AccountId), Balance>,
    }
    #[ink(event)]
    pub struct Transfer
    {
        #[ink(topic)]
        from:AccountId,

        #[ink(topic)]
        to:AccountId,
        #[ink(topic)]
        value:Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    #[derive(Debug, PartialEq,Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {

        InsufficientBalance,
        InsufficientAllowance,
    }

    pub type Result<T> = core::result::Result<T,Error>;

    impl Erc20 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = StorageHashMap::new();
            balances.insert(caller, total_supply);
            let instance = Self{
                total_supply:total_supply,
                balances:balances,
                allowance:StorageHashMap::new(),
            };
            instance
        }

        ///公共方法 1读 2写
        #[ink(message)]
        pub fn total_supply(&self) -> Balance{
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner:AccountId)->Balance{
            *self.balances.get(&owner).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to:AccountId, value:Balance)->Result<()>
        {
            let who = Self::env().caller();
            self.transfer_help(who,to,value)
        }

        fn transfer_help(&mut self, from:AccountId, to:AccountId, value:Balance)->Result<()>{

            let from_balance = self.balance_of(from);
            if from_balance < value{
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(from,from_balance - value);
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);

            self.env().emit_event(
                Transfer{
                    from:from,
                    to:to,
                    value:value
                }    
            );
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()>{
            let caller = Self::env().caller();
            let allowance = self.allowance(from, caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance)
            }
            self.transfer_from_to(from, to , value)?;

            self.allowance.insert((from, to), allowance - value);
            
            Ok(())
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value:Balance) -> Result<()>{


            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance)
            }
            self.balances.insert(from, from_balance - value);

            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + value);
            
            self.env().emit_event( Transfer{
                from : from,
                to : to,
                value : value
            });
            Ok(())
        }
        // #[ink(message)]
        // pub fn transfer_from(&mut self, from:AccountId, to:AccountId, value:Balance) -> Reslult
        // {


        // }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()>{
            let caller = Self::env().caller();
            self.allowance.insert((caller, spender), value);

            self.env().emit_event( Approval{
                owner : caller,
                spender : spender,
                value : value,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
 
            *self.allowance.get(&(owner, spender)).unwrap_or(&0)
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
        fn create_contract_works() {
            let erc20 = Erc20::default();
            assert_eq!(erc20.get(), false);
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
            let mut erc20 = Erc20::new(false);
            assert_eq!(erc20.get(), false);
            erc20.flip();
            assert_eq!(erc20.get(), true);
        }
    }
}
