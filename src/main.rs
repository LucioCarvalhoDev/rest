use std::collections::HashMap;
use chrono::NaiveDate;

fn main() {
    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        id: i32,
    }

    #[derive(Debug, Clone)]
    struct Account {
        title: String,
        id: i32,
    }

    #[derive(Debug, Clone)]
    enum TransactionRepeatType {
        EveryDate(String),
        EveryXDays(i32),
    }

    #[derive(Debug, Clone)]
    struct Transaction {
        value: f64,
        target_account: i32,
        source_account: Option<i32>,
        date: NaiveDate,
        repeat: Option<bool>,
        repeat_mode: Option<TransactionRepeatType>,
        auto_debt: Option<bool>
    }

    #[derive(Debug, Clone)]
    struct FinanceManager {
        pub persons: Vec<Person>,
        pub accounts: Vec<Account>,
        pub transactions: HashMap<Vec<Transaction>>,
        pub next_id: i32
    }

    impl FinanceManager {
        fn new() -> FinanceManager {
            let mut fin_manager = FinanceManager {
                persons: Vec::new(),
                accounts: Vec::new(),
                transactions: Vec::new(),
                next_id: 1
            };

            fin_manager.create_person(String::from("MainUser"));

            fin_manager
        }

        fn generate_id(&mut self) -> i32 {
            let id = self.next_id;
            self.next_id += 1;
            id
        }

        fn create_person(&mut self, name: String) -> i32 {
            let prs = Person {
                id: self.generate_id(),
                name: name,
            };

            let prs_id = prs.id;

            self.persons.push(prs);
            
            return prs_id;

        }

        fn create_account(&mut self, title: String) -> i32 {
            let buff = Account {
                title: title,
                id: self.generate_id(),
            };

            let buff_id = buff.id;

            self.accounts.push(buff);

            return buff_id
        }
    }

    let core = FinanceManager::new();
    println!("{:?}", core);
}
