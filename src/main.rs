pub mod atoms;
pub mod finance_manager;

use atoms::atoms::{ID};
use finance_manager::fm::FinanceManager;

fn main() {   
    struct View;

    impl View {
        fn display_balance(fm: &FinanceManager, transactions: Vec<ID>) {
            for (account_id, transactions_id) in fm.organize_transactions_by_account(transactions) {
                println!("{:?}  ${:?}", account_id, fm.calc_transactions(transactions_id));
            };
        }
    }
    
    let core = FinanceManager::new();

    View::display_balance(&core, core.get_transactions_from_relative_day(0));
}
