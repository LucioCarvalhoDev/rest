pub mod atoms {
    use chrono::{DateTime, Utc};

    #[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
    pub enum ID {
        PersonID(i32),
        AccountID(i32),
        TransactionID(i32),
    }

    impl ID {
        pub fn get_value(&self) -> i32 {
            return match self {
                ID::PersonID(id) => *id,
                ID::AccountID(id) => *id,
                ID::TransactionID(id) => *id,
            }
        }
    }

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

        pub fn person(&mut self) -> ID {
            let id = self._person;
            self._person += 1;
            return ID::PersonID(id);
        }

        pub fn account(&mut self) -> ID {
            let id = self._account;
            self._account += 1;
            return ID::AccountID(id);
        }

        pub fn transaction(&mut self) -> ID {
            let id = self._transaction;
            self._transaction += 1;
            return ID::TransactionID(id);
        }
    }

    #[derive(Debug, Clone)]
    pub struct Person {
        pub name: String,
        pub id: ID,
        pub accounts: Vec<i32>
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Account {
        pub title: String,
        pub id: ID,
        pub owners: Vec<i32>,
    }

    #[derive(Debug, Clone)]
    pub enum TransactionRepeatType {
        EveryDate(String),
        EveryXDays(i32),
    }

    #[derive(Debug, Clone)]
    pub struct Transaction {
        pub id: ID,
        pub value: f64,
        pub target_account: i32,
        pub source_account: Option<i32>,
        pub date: DateTime<Utc>,
    }
}
