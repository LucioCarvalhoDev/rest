pub mod fm {
    use crate::atoms::atoms::{Account, ID, IdGenerator, Person, Transaction};
    use chrono::{DateTime, Datelike, NaiveDate, Utc};
    use std::{collections::HashMap, vec};

    #[derive(Debug, Clone)]
    pub struct FinanceManager {
        pub persons: HashMap<i32, Person>,
        pub accounts: HashMap<i32, Account>,
        pub transactions: HashMap<NaiveDate, Vec<Transaction>>,
        id_generator: IdGenerator,
    }

    impl FinanceManager {
        pub fn new() -> FinanceManager {
            let mut fin_manager = FinanceManager {
                persons: HashMap::new(),
                accounts: HashMap::new(),
                transactions: HashMap::new(),
                id_generator: IdGenerator::new(),
            };

            fin_manager.create_person("MainUser");
            fin_manager.create_account("wallet");

            fin_manager
        }

        pub fn create_person(&mut self, name: &str) {
            let prs = Person {
                id: self.id_generator.person(),
                name: name.to_string(),
                accounts: vec![],
            };

            self.persons.insert(prs.id.get_value(), prs);
        }

        pub fn create_account(&mut self, title: &str) {
            let acc = Account {
                title: title.to_string(),
                id: self.id_generator.account(),
                owners: vec![1],
            };

            for owner_id in acc.owners.iter() {
                match self.persons.get_mut(owner_id) {
                    Some(owner) => owner.accounts.push(acc.id.get_value()),
                    None => println!("Person não encontrada"),
                };
            }
            self.accounts.insert(acc.id.get_value(), acc);
        }

        pub fn get_person_id_by_name(&self, name: &str) -> Option<i32> {
            let mut id = None;
            for person in self.persons.iter() {
                if person.1.name == name {
                    id = Some(*person.0);
                    break;
                }
            }
            return id;
        }

        pub fn get_person_by_id(&self, id: i32) -> Option<&Person> {
            self.persons.get(&id)
        }

        pub fn list_accounts_from(&self, person_id: ID) -> Vec<ID> {
            let mut res: Vec<ID> = vec![];
            match self.persons.get(&person_id.get_value()) {
                Some(person) => {
                    person
                        .accounts
                        .iter()
                        .for_each(|x| res.push(ID::AccountID(*x)));
                }
                None => (),
            };

            return res;
        }

        pub fn create_deposit(&mut self, target_id: i32, amount: f64) {
            let trs = Transaction {
                value: amount,
                id: self.id_generator.transaction(),
                target_account: target_id,
                source_account: None,
                date: Utc::now(),
            };

            let date = trs.date.date_naive();

            self.transactions
                .entry(date)
                .or_insert_with(|| vec![])
                .push(trs);
        }

        pub fn extract_date(when: chrono::DateTime<Utc>) -> NaiveDate {
            let date =
                NaiveDate::from_ymd_opt(when.year(), when.month(), when.day()).expect("date");
            return date;
        }

        pub fn organize_transactions_by_account(&self, list: Vec<ID>) -> HashMap<ID, Vec<ID>> {
            let mut balance: HashMap<ID, Vec<ID>> = HashMap::new();
            
            for trs_id in list.iter() {
                if let Some(trs) = self.get_transaction_by_id(*trs_id) {
                    balance
                        .entry(ID::AccountID(trs.target_account))
                        .or_insert(vec![])
                        .push(trs.id);
                };
                
            }

            return balance;
        }


        pub fn get_transaction_by_id(&self, id: ID) -> Option<&Transaction> {
            match id {
                ID::TransactionID(_) => {
                    for transactions in self.transactions.values() {
                        if let Some(transaction) = transactions.iter().find(|t| t.id == id) {
                            return Some(transaction);
                        }
                    }
                    None
                }
                ID::PersonID(_) => panic!(),
                ID::AccountID(_) => panic!(),
            }
        }

        pub fn calc_transactions(&self, list: Vec<ID>) -> f64 {
            let mut sum = 0.0;
            for trs_id in list.iter() {
                if let Some(trs) = self.get_transaction_by_id(*trs_id) {
                    sum += trs.value;
                }
            }

            return sum;
        }

        pub fn reduce_balance(&self, balance: HashMap<ID, Vec<ID>>) {
            for (account, transactions) in balance {
                println!(
                    "{} ${}",
                    account.get_value(),
                    self.calc_transactions(transactions)
                )
            }
        }

        pub fn create_transaction(&mut self, target_account: i32, date: DateTime<Utc>, amount: f64) -> ID { 
            let trs = Transaction {
                id: self.id_generator.transaction(),
                value: amount,
                target_account: target_account,
                source_account: None,
                date: date,
            };

            let id = trs.id;

            self.transactions
                .entry(FinanceManager::extract_date(date))
                .or_insert(vec![])
                .push(trs);

            return id;
        }

        pub fn get_transactions_from_relative_day(&self, offset: i64) -> Vec<ID> {
            let mut res = vec![];
            let today = FinanceManager::extract_date(Utc::now());
            match self.transactions
                .get(&(today + chrono::Duration::days(offset))) {
                    Some(list) => {
                        list.iter().for_each(|x| {res.push(x.id)});
                    },
                    None => (),
                };
            return res;
        }
    }
}
