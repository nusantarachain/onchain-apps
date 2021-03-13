//! nuchain-erc20
//! 
//! ERC-20 Implementation for Nuchain.
//! 
//! Copyright (C) 2021 Robin Syihab <obin.mf@gmail.com>
//! 
//! My ERC-20 Smart contract.
//! 
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod nuchain_erc20 {

    use ink_prelude::string;

    /// Token unit type
    type Token = u32;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[cfg(not(feature = "ink-as-dependency"))]
    #[ink(storage)]
    pub struct NuchainErc20 {
        /// The total supply of the contract.
        total_supply: Token,

        /// Token of each user's contract.
        balances: ink_storage::collections::HashMap<AccountId, Token>,

        /// The owner of this contract.
        owner: AccountId,

        /// Tokens that are spendable by non-owners
        /// this useful for subscription model payment.
        allowances: ink_storage::collections::HashMap<(AccountId, AccountId), Token>
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        amount: Token,
    }

    #[ink(event)]
    pub struct Failed {
        #[ink(topic)]
        msg: string::String
    }

    #[ink(event)]
    pub struct Approval {
        /// Owner of the balances.
        owner: AccountId,
        
        /// Spender who permitted to transfer.
        spender: AccountId,

        /// Amount balance allowed to transfer by spender.
        amount: Token,
    }

    impl NuchainErc20 {
        /// Constructor that initializes the total supply value to the given `total_supply`.
        #[ink(constructor)]
        pub fn new(total_supply: Token) -> Self {
            let mut balances = ink_storage::collections::HashMap::new();
            let owner = Self::env().caller();
            balances.insert(owner, total_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(owner.clone()),
                amount: total_supply,
            });

            Self {
                total_supply,
                balances,
                owner,
                allowances: Default::default(),
            }
        }

        /// Default constructor when no initial supply specified.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(21000000)
        }

        /// Token symbol
        #[ink(message)]
        pub fn symbol(&self) -> &'static str {
            "NUC"
        }

        /// Get the total supply of token of this contract.
        #[ink(message)]
        pub fn total_supply(&self) -> Token {
            self.total_supply
        }

        /// Transfer token
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: Token) -> bool {
            self.transfer_from_to(self.env().caller(), to, amount)
        }

        /// Spend token by permission of the owner.
        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, amount: Token) -> bool {
            let allowance = self.get_allowance_or_zero(&from, &self.env().caller());
            if allowance < amount {
                // amount to transfer is more than allowed.
                self.env().emit_event(Failed {
                    msg: "Amount is more than allowed".into(),
                });
                return false;
            }

            self.allowances.insert((from, self.env().caller()), allowance - amount);
            self.transfer_from_to(from, to, amount);

            true
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, amount: Token) -> bool {
            let sender_bal = self.get_balance_or_zero(&from);
            if sender_bal < amount {
                // insufficient amount
                self.env().emit_event(Failed {
                    msg: "Insufficient amount".into(),
                });
                return false;
            }

            self.balances.insert(from, sender_bal - amount);

            let receiver_bal = self.get_balance_or_zero(&to);
            self.balances.insert(to, receiver_bal + amount);

            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                amount,
            });

            true
        }

        /// Get account's balances
        #[ink(message)]
        pub fn balance_of(&self, of: AccountId) -> Token {
            self.get_balance_or_zero(&of)
        }

        /// Approve third party to spend on behalf of user's account
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, amount: Token) -> bool {
            let owner = self.env().caller();

            self.allowances.insert((owner, spender), amount);

            self.env().emit_event(Approval {
                owner: owner,
                spender: spender,
                amount: amount,
            });

            true
        }

        /// Get remaining balance allowed by owner to spend by third party.
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Token {
            self.get_allowance_or_zero(&owner, &spender)
        }

        fn get_balance_or_zero(&self, of: &AccountId) -> Token {
            *self.balances.get(of).unwrap_or(&0)
        }

        fn get_allowance_or_zero(&self, owner: &AccountId, spender: &AccountId) -> Token {
            *self.allowances.get(&(*owner, *spender)).unwrap_or(&0)
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let contract = NuchainErc20::new(2100);
            assert_eq!(contract.total_supply(), 2100);
        }

        
        #[ink::test]
        fn default_works() {
            let contract = NuchainErc20::default();
            assert_eq!(contract.total_supply(), 21000000);
        }

        #[ink::test]
        fn balance_works() {
            let contract = NuchainErc20::new(100);
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = NuchainErc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
            contract.transfer(AccountId::from([0x0; 32]), 10);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 90);
            contract.transfer(AccountId::from([0x0; 32]), 10);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 20);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 80);
        }

        #[ink::test]
        fn transfer_from_works(){
            let mut contract = NuchainErc20::new(100);
            contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x2; 32]), 10);
            assert_eq!(contract.balance_of(AccountId::from([0x2; 32])), 0);

            // now approve 0x1
            contract.approve(AccountId::from([0x1; 32]), 10);

            contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x2; 32]), 5);
            assert_eq!(contract.balance_of(AccountId::from([0x2; 32])), 5);

            // cannot spend more than allowed
            contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x2; 32]), 20);
            assert_eq!(contract.balance_of(AccountId::from([0x2; 32])), 5);
            contract.transfer_from(AccountId::from([0x1; 32]), AccountId::from([0x2; 32]), 5);
            assert_eq!(contract.balance_of(AccountId::from([0x2; 32])), 10);
        }

        #[ink::test]
        fn allowance_works() {
            let mut contract = NuchainErc20::new(100);
            assert_eq!(
                contract.allowance(AccountId::from([0x1; 32]), AccountId::from([0x2; 32])),
                0
            );
            contract.approve(AccountId::from([0x2; 32]), 10);
            assert_eq!(
                contract.allowance(AccountId::from([0x1; 32]), AccountId::from([0x2; 32])),
                10
            );
        }

        #[ink::test]
        fn get_token_symbol(){
            let contract = NuchainErc20::new(100);
            assert_eq!(contract.symbol(), "NUC");
        }
    }
}

