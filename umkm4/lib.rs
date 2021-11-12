//! UMKM4
//!
//! Aplikasi smart contract loyality program sederhana yang jalan
//! di atas jaringan Blockchain Nuchain.
//!
//!
//! UMKM 4.0
//!
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod umkm4 {

    use ink_prelude::{string::String, *};
    use ink_storage::traits::{
        PackedLayout,
        SpreadLayout,
    };

    // macro untuk mempermudah pembuatan struktur yang 
    // kompatibel untuk native dan WebAssembly.
    macro_rules! def_struct {
        ($name:ident, { $( $field:ident : $t:ty, )* } ) => {
            #[derive(PackedLayout, SpreadLayout, scale::Encode, scale::Decode, Clone)]
            #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, Debug))]
            pub struct $name {
                $(pub $field : $t,)*
            }
        };
    }

    // Membership data
    def_struct!(MemberData, {
        name: String,
        point: u32,
        used_point: u32,
    });

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[cfg(not(feature = "ink-as-dependency"))]
    #[ink(storage)]
    pub struct Umkm4 {
        /// The owner of this contract.
        owner: AccountId,

        /// Admin untuk contract ini
        admin: AccountId,

        /// membership data
        /// pairing AccountId -> (points, used_points)
        members: ink_storage::collections::HashMap<Hash, MemberData>,

        /// for membership id generator
        member_index: u32,

        /// Member hash pair
        members_hash: ink_storage::collections::HashMap<u32, Hash>,
    }

    #[ink(event)]
    pub struct Failed {
        #[ink(topic)]
        msg: string::String,
    }

    #[ink(event)]
    pub struct PointUsed {
        pub user_id: u32,
        pub amount: u32,
    }

    impl Umkm4 {
        /// Konstruktor pertama untuk inisasi smart contract ketika pertama kali
        /// dideploy ke jaringan Nuchain.
        #[ink(constructor)]
        pub fn new(owner: AccountId) -> Self {
            let initiator = Self::env().caller();

            Self {
                owner,
                admin: initiator,
                members: Default::default(),
                member_index: 0,
                members_hash: Default::default(),
            }
        }

        /// Register new member, return new registered user ID
        #[ink(message)]
        pub fn register(&mut self, name: String) -> Option<(Hash, u32)> {
            let caller = Self::env().caller();
            if caller != self.owner && caller != self.admin {
                return None;
            }
            let id = self.member_index + 1;
            let mut subject = vec![id as u8];
            subject.extend(name.bytes());
            let (hash_id, _) = Self::env().random(&subject);
            self.members.insert(
                hash_id,
                MemberData {
                    name,
                    point: 0,
                    used_point: 0,
                },
            );
            self.members_hash.insert(id, hash_id);
            self.member_index = id;
            Some((hash_id, id))
        }

        /// Get last member index
        #[ink(message)]
        pub fn last_id(&self) -> u32 {
            self.member_index
        }

        /// Get total member count
        #[ink(message)]
        pub fn get_member_count(&self) -> u32 {
            self.members.len()
        }

        #[ink(message)]
        pub fn hash_caller(&self) -> AccountId {
            let caller: AccountId = self.env().caller();
            // self.env().hash_encoded::<ink_env::hash::Blake2x256, _>(&caller).into()
            caller
        }
        

        /// Get member data
        #[ink(message)]
        pub fn get_member(&self, id: Hash) -> Option<MemberData> {
            self.members.get(&id).map(|a| a.clone())
        }

        /// Get member data
        #[ink(message)]
        pub fn get_member_by_index(&self, index: u32) -> Option<MemberData> {
            let caller:AccountId = Self::env().caller();
            if caller != self.owner {
                return None;
            }
            self.get_member_hash(index)
                .and_then(|h| self.members.get(&h).map(|a| a.clone()))
        }

        /// Get member hash by index.
        #[ink(message)]
        pub fn get_member_hash(&self, index: u32) -> Option<Hash> {
            let caller:AccountId = Self::env().caller();
            if caller != self.owner {
                return None;
            }
            self.members_hash.get(&index).map(|a| a.clone())
        }

        /// Increase account point
        #[ink(message)]
        pub fn add_point(&mut self, id: Hash, amount: u32) -> i32 {
            let caller = Self::env().caller();
            if caller != self.owner {
                return -1;
            }
            if let Some(m) = self.members.get_mut(&id) {
                m.point = m.point.saturating_add(amount);
                m.point as i32
            } else {
                0
            }
        }

        /// Gunakan point yang ada pada suatu akun
        #[ink(message)]
        pub fn use_point(&mut self, id: Hash, amount: u32) -> i32 {
            let caller = Self::env().caller();
            if caller != self.owner {
                return -1;
            }
            if let Some(m) = self.members.get_mut(&id) {
                let point = m.point.clone();
                if m.point < amount {
                    self.env().emit_event(Failed {
                        msg: format!("Not enough point {} < {}", point, amount),
                    });
                    return 0;
                }
                m.point = m.point.saturating_sub(amount);
                m.used_point = m.used_point.saturating_add(amount);
                m.point as i32
            } else {
                0
            }
        }

        /// Apabila ingin mengganti owner dari instansi kontrak ini.
        /// hanya admin yang bisa melakukan ini.
        #[ink(message)]
        pub fn set_owner(&mut self, new_owner: AccountId) {
            let caller = Self::env().caller();
            if caller != self.admin {
                return;
            }
            self.owner = new_owner;
        }

        /// Mendapatkan akun owner aktif saat ini.
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        /// Mendapatkan akun admin instansi kontrak ini.
        #[ink(message)]
        pub fn get_admin(&self) -> AccountId {
            self.admin
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
            let ac2 = AccountId::from([0x2; 32]);
            let contract = Umkm4::new(ac2);
            assert_eq!(contract.get_admin(), AccountId::from([0x1; 32]));
            assert_eq!(contract.get_owner(), ac2);
        }

        fn new_contract() -> Umkm4 {
            let owner = AccountId::from([0x2; 32]);
            let contract = Umkm4::new(owner);
            contract
        }

        #[ink::test]
        fn register_works() {
            let mut contract = new_contract();
            let (hash, id) = contract.register("USER1".to_string()).unwrap();
            set_sender([0x2; 32].into());
            assert_eq!(contract.get_member_hash(id), Some(hash));
            assert!(contract.get_member_by_index(id).is_some());
            assert!(contract.get_member_by_index(id + 1).is_none());
        }

        #[ink::test]
        fn add_point_works() {
            let mut contract = new_contract();
            set_sender([0x2; 32].into());
            let (hash, id) = contract.register("USER1".to_string()).unwrap();
            contract.add_point(hash, 5);
            assert_eq!(contract.get_member(hash).map(|a| a.point), Some(5));
            contract.add_point(hash, 5);
            assert_eq!(contract.get_member_by_index(id).map(|a| a.point), Some(10));
        }

        #[ink::test]
        fn use_point_works() {
            let mut contract = new_contract();
            set_sender([0x2; 32].into());
            let (hash, _id) = contract.register("USER1".to_string()).unwrap();
            contract.add_point(hash, 5);
            assert_eq!(contract.get_member(hash).map(|a| a.point), Some(5));
            contract.use_point(hash, 3);
            assert_eq!(contract.get_member(hash).map(|a| a.point), Some(2));
        }

        #[ink::test]
        fn set_owner_works() {
            let mut contract = new_contract();
            assert_eq!(contract.get_owner(), [0x2; 32].into());
            contract.set_owner([0x3; 32].into());
            assert_eq!(contract.get_owner(), [0x3; 32].into());
        }

        #[ink::test]
        fn only_owner_can_add_point() {
            let mut contract = new_contract();
            let (hash, _id) = contract.register("USER1".to_string()).unwrap();
            set_sender([0x3; 32].into());
            contract.add_point(hash, 5);
            assert_eq!(contract.get_member(hash).map(|a| a.point), Some(0));
        }

        #[ink::test]
        fn only_owner_can_use_point() {
            let mut contract = new_contract();
            let (hash, _id) = contract.register("USER1".to_string()).unwrap();
            set_sender([0x2; 32].into());
            contract.add_point(hash, 5);
            assert_eq!(contract.get_member(hash).map(|a| a.point), Some(5));
            set_sender([0x3; 32].into());
            contract.use_point(hash, 5);
            assert_eq!(contract.get_member(hash).map(|a| a.point), Some(5));
        }

        /// Helper function digunakan untuk mengganti akun pemanggil fungsi.
        fn set_sender(sender: AccountId) {
            let callee =
                ink_env::account_id::<ink_env::DefaultEnvironment>().unwrap_or([0x0; 32].into());
            ink_env::test::push_execution_context::<Environment>(
                sender,
                callee,
                1000000,
                1000000,
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])), // dummy
            );
        }
    }
}
