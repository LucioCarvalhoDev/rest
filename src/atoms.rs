pub mod atoms {
    use chrono::{DateTime, Utc};

    // #[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
    // pub enum ID {
    //     PersonID(i32),
    //     AccountID(i32),
    //     TransactionID(i32),
    // }

    // impl ID {
    //     pub fn get_value(&self) -> i32 {
    //         return match self {
    //             ID::PersonID(id) => *id,
    //             ID::AccountID(id) => *id,
    //             ID::TransactionID(id) => *id,
    //         }
    //     }

    //     pub fn is_person(&self) -> bool {
    //         return *self == ID::PersonID(self.get_value())
    //     }

    //     pub fn is_account(&self) -> bool {
    //         return *self == ID::AccountID(self.get_value())
    //     }

    //     pub fn is_transaction(&self) -> bool {
    //         return *self == ID::TransactionID(self.get_value())
    //     }
    // }

    #[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
    pub struct PersonID(pub i32);

    #[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
    pub struct AccountID(pub i32);

    #[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
    pub struct TransactionID(pub i32);

    #[derive(Debug, Clone)]
    pub struct IdGenerator {
        _person: i32,
        _account: i32,
        _transaction: i32,
    }

    impl IdGenerator {
        pub fn new() -> IdGenerator {
            IdGenerator {
                _person: 1,
                _account: 1,
                _transaction: 1,
            }
        }

        pub fn person(&mut self) -> PersonID {
            let id = self._person;
            self._person += 1;
            return PersonID(id);
        }

        pub fn account(&mut self) -> AccountID {
            let id = self._account;
            self._account += 1;
            return AccountID(id);
        }

        pub fn transaction(&mut self) -> TransactionID {
            let id = self._transaction;
            self._transaction += 1;
            return TransactionID(id);
        }
    }

    #[derive(Debug, Clone)]
    pub struct Person {
        pub name: String,
        pub id: PersonID,
        pub accounts: Vec<AccountID>
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Account {
        pub title: String,
        pub id: AccountID,
        pub owners: Vec<PersonID>,
    }

    #[derive(Debug, Clone)]
    pub enum TransactionRepeatType {
        EveryDate(String),
        EveryXDays(i32),
    }

    #[derive(Debug, Clone)]
    pub struct Transaction {
        pub id: TransactionID,
        pub value: f64,
        pub target_account: AccountID,
        pub source_account: Option<AccountID>,
        pub date: DateTime<Utc>,
    }
}
