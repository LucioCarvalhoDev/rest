pub mod atoms;

use atoms::atoms::{ID, IdGenerator, Account, Person, Transaction};
use std::collections::HashMap;
use chrono::{Datelike, Local, NaiveDate, Utc};

fn main() {   
    #[derive(Debug, Clone)]
    struct FinanceManager {
        pub persons: HashMap<i32, Person>,
        pub accounts: HashMap<i32, Account>,
        pub transactions: HashMap<NaiveDate, Vec<Transaction>>,
        id_generator: IdGenerator
    }

    impl FinanceManager {
        fn new() -> FinanceManager {
            let mut fin_manager = FinanceManager {
                persons: HashMap::new(),
                accounts: HashMap::new(),
                transactions: HashMap::new(),
                id_generator: IdGenerator::new()
            };

            fin_manager.create_person("MainUser");
            fin_manager.create_account("wallet");

            fin_manager
        }

        fn create_person(&mut self, name: &str)  {
            let prs = Person {
                id: self.id_generator.person(),
                name: name.to_string(),
                accounts: vec![],
            };

            self.persons.insert(prs.id.get_value(), prs);

        }

        fn create_account(&mut self, title: &str) {
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

        fn get_person_id_by_name(&self, name: &str) -> Option<i32> {
            let mut id = None;
            for person in self.persons.iter() {
                if person.1.name == name {
                    id = Some(*person.0);
                    break;
                }
            }
            return id;
        }

        fn get_person_by_id(&self, id: i32) -> Option<&Person> {
            self.persons.get(&id)
        }

        fn list_accounts_from(&self, person_id: ID) -> Vec<ID> {
            let mut res: Vec<ID> = vec![];
            match self.persons.get(&person_id.get_value()) {
                Some(person) => {
                    person.accounts.iter().for_each(|x| res.push(ID::AccountID(*x)));
                },
                None => (),
            };

            return res;
        }

        fn create_deposit(&mut self, target_id: i32, amount: f64) {
            let trs = Transaction {
                value: amount,
                id: self.id_generator.transaction(),
                target_account: target_id,
                source_account: None,
                date: Utc::now(),
            };

            let date = trs.date.date_naive();

            self.transactions.entry(date)
                .or_insert_with(|| {vec![]})
                .push(trs);
        }

        fn extract_date(&self, when: chrono::DateTime<Local>) -> NaiveDate {
            let date = NaiveDate::from_ymd_opt(when.year(), when.month(), when.day()).expect("date");
            return date
        }

        fn get_day_balance(&self, date: NaiveDate) -> HashMap<ID, Vec<ID>> {
            let mut balance: HashMap<ID, Vec<ID>> = HashMap::new();
            match self.transactions.get(&date) {
                Some(list) => {

                    for transaction in list.iter() {
                        if let Some(account) = self.accounts.get(&transaction.target_account) {
                            balance.entry(account.id)
                                .or_insert_with(|| {vec![]})
                                .push(transaction.id)
                        }
                    }
                },
                None => todo!(),
            }

            return balance;
        }

        fn get_transaction_by_id(&self, id: ID) -> Option<&Transaction> {
            match id {
                ID::TransactionID(_) => {
                    for transactions in self.transactions.values() {
                        if let Some(transaction) = transactions.iter().find(|t| t.id == id) {
                            return Some(transaction)
                        }
                    }
                    None
                },
                ID::PersonID(_) => panic!(),
                ID::AccountID(_) => panic!(),
            }
        }

        fn calc_transactions(&self, list: Vec<ID>) -> f64 {
            let mut sum = 0.0;
            for trs_id in list.iter() {
                if let Some(trs) = self.get_transaction_by_id(*trs_id) {
                    sum += trs.value;
                }

            }

            return sum;
        }

        fn reduce_balance(&self, balance: HashMap<ID, Vec<ID>>) {
            for (account, transactions) in balance {
                println!("{} ${}", account.get_value(), self.calc_transactions(transactions))
            }
        }

    }

    struct View;

    impl View {
        fn display_day_balance(fm: &FinanceManager) {
            for (account_id, transactions_id) in fm.get_day_balance(Utc::now().date_naive()) {
                println!("{} $ {}", fm.accounts.get(&account_id.get_value()).unwrap().title, fm.calc_transactions(transactions_id))
            };
        }
    }
    
    let mut core = FinanceManager::new();

    core.create_account("nubank");
    core.create_deposit(1, 50.0);
    core.create_account("bb");
    core.create_deposit(3, -51.0);
    core.create_deposit(1, -13.90);
    core.create_deposit(2, 1723.5);
    core.create_deposit(1, 50.0);
    core.create_deposit(3, -51.0);

    //println!("{:?}", core.persons.get(&1));
    //println!("{:?}", core.list_accounts_from(ID::PersonID(1)));
    //println!("{:?}", core.get_transaction_by_id(ID::TransactionID(1)));

    //println!("{:?}", core.get_day_balance(Utc::now().date_naive()));
    View::display_day_balance(&core);
}
