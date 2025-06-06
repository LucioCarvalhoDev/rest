pub mod fm {
    use crate::atoms::atoms::{
        Account, AccountID, IdGenerator, Person, PersonID, Transaction, TransactionID,
    };
    use chrono::{DateTime, Datelike, NaiveDate, Utc};
    use std::{collections::HashMap, vec};

    #[derive(Debug, Clone)]
    pub struct FinanceManager {
        pub persons: HashMap<PersonID, Person>,
        pub accounts: HashMap<AccountID, Account>,
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

            // fin_manager.create_person("MainUser");
            // fin_manager.create_account_for_persons("wallet", vec![PersonID(1)]);

            fin_manager
        }

        // pub fn get_all_data_from_person(&self) {
        //     unimplemented!();
        // }

        // pub fn create_person(&mut self, name: &str) {
        //     let prs = Person {
        //         id: self.id_generator.person(),
        //         name: name.to_string(),
        //     };

        //     self.persons.insert(prs.id, prs);
        // }

        // pub fn create_account_for_persons(&mut self, title: &str, ids: Vec<PersonID>) {
        //     let acc = Account {
        //         title: title.to_string(),
        //         id: self.id_generator.account(),
        //     };

        //     for owner_id in acc.owners.iter() {
        //         match self.persons.get_mut(owner_id) {
        //             Some(owner) => owner.accounts.push(acc.id),
        //             None => println!("Person não encontrada"),
        //         };
        //     }
        //     self.accounts.insert(acc.id, acc);
        // }

        // pub fn get_person_id_by_name(&self, name: &str) -> Option<PersonID> {
        //     let mut id = None;
        //     for person in self.persons.iter() {
        //         if person.1.name == name {
        //             id = Some(*person.0);
        //             break;
        //         }
        //     }
        //     return id;
        // }

        // pub fn get_person_by_id(&self, id: PersonID) -> Option<&Person> {
        //     self.persons.get(&id)
        // }

        // pub fn list_accounts_from(&self, person_id: PersonID) -> Vec<AccountID> {
        //     let mut res: Vec<AccountID> = vec![];
        //     match self.persons.get(&person_id) {
        //         Some(person) => {
        //             person.accounts.iter().for_each(|x| res.push(*x));
        //         }
        //         None => (),
        //     };

        //     return res;
        // }

        // pub fn create_deposit(&mut self, target_id: AccountID, amount: f64) {
        //     let trs = Transaction {
        //         value: amount,
        //         id: self.id_generator.transaction(),
        //         target_account: target_id,
        //         source_account: None,
        //         date: Utc::now(),
        //     };

        //     let date = trs.date.date_naive();

        //     self.transactions
        //         .entry(date)
        //         .or_insert_with(|| vec![])
        //         .push(trs);
        // }

        // pub fn extract_date(when: chrono::DateTime<Utc>) -> NaiveDate {
        //     let date =
        //         NaiveDate::from_ymd_opt(when.year(), when.month(), when.day()).expect("date");
        //     return date;
        // }

        // pub fn organize_transactions_by_account(
        //     &self,
        //     list: Vec<TransactionID>,
        // ) -> HashMap<AccountID, Vec<TransactionID>> {
        //     let mut balance: HashMap<AccountID, Vec<TransactionID>> = HashMap::new();

        //     for trs_id in list.iter() {
        //         if let Some(trs) = self.get_transaction_by_id(*trs_id) {
        //             balance
        //                 .entry(trs.target_account)
        //                 .or_insert(vec![])
        //                 .push(trs.id);
        //         };
        //     }

        //     return balance;
        // }

        // pub fn get_transaction_by_id(&self, id: TransactionID) -> Option<&Transaction> {
        //     for transactions in self.transactions.values() {
        //         if let Some(transaction) = transactions.iter().find(|t| t.id == id) {
        //             return Some(transaction);
        //         }
        //     }
        //     None
        // }

        // pub fn calc_transactions(&self, list: Vec<TransactionID>) -> f64 {
        //     let mut sum = 0.0;
        //     for trs_id in list.iter() {
        //         if let Some(trs) = self.get_transaction_by_id(*trs_id) {
        //             sum += trs.value;
        //         }
        //     }

        //     return sum;
        // }

        // pub fn reduce_balance(&self, balance: HashMap<AccountID, Vec<TransactionID>>) {
        //     for (account, transactions) in balance {
        //         println!("{} ${}", account.0, self.calc_transactions(transactions))
        //     }
        // }

        // pub fn create_transaction(
        //     &mut self,
        //     target_account: AccountID,
        //     date: DateTime<Utc>,
        //     amount: f64,
        // ) -> TransactionID {
        //     let trs = Transaction {
        //         id: self.id_generator.transaction(),
        //         value: amount,
        //         target_account: target_account,
        //         source_account: None,
        //         date: date,
        //     };

        //     let id = trs.id;

        //     self.transactions
        //         .entry(FinanceManager::extract_date(date))
        //         .or_insert(vec![])
        //         .push(trs);

        //     return id;
        // }

        // pub fn get_transactions_from_relative_day(&self, offset: i64) -> Vec<TransactionID> {
        //     let mut res = vec![];
        //     let today = FinanceManager::extract_date(Utc::now());
        //     match self
        //         .transactions
        //         .get(&(today + chrono::Duration::days(offset)))
        //     {
        //         Some(list) => {
        //             list.iter().for_each(|x| res.push(x.id));
        //         }
        //         None => (),
        //     };
        //     return res;
        // }
    }
}
