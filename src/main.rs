pub mod atoms;
pub mod finance_manager;

use atoms::atoms::{TransactionID};
use finance_manager::fm::FinanceManager;

fn main() {
    struct View {
        fm: FinanceManager
    }

    impl View {
        fn display_balance(&self, transactions: Vec<TransactionID>) {
            for (account_id, transactions_id) in self.fm.organize_transactions_by_account(transactions) {
                println!("{:?}  ${:?}", account_id, self.fm.calc_transactions(transactions_id));
            };
        }
    }

}
