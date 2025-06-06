use rusqlite::Connection;

use crate::{
    atoms::atoms::{Person, PersonID},
    finance_manager::fm::FinanceManager,
};

pub mod atoms;
pub mod finance_manager;

fn main() {
    let _ = std::fs::write("./data/debug.db", "");

    struct App {
        cnn: Connection,
        fm: FinanceManager,
    }

    impl App {
        fn new() -> App {
            match rusqlite::Connection::open("./data/rest.db") {
                Ok(connection) => {
                    let app = App {
                        cnn: connection,
                        fm: FinanceManager::new(),
                    };
                    println!("Banco de dados acessado com sucesso");
                    return app;
                }
                Err(_) => panic!("Banco de dados não encontrado"),
            }
        }

        //    fn load_persons(&self) {
        //         let mut stmt = self.cnn.prepare("SELECT * FROM Persons").unwrap();
        //         stmt.query_map([], |row| {
        //             Ok(Person {
        //                 id: PersonID(row.get::<usize, i32>(0).unwrap()),
        //                 name: row.get::<usize, String>(1).unwrap()
        //             })
        //         }).unwrap();
        //         println!("{:?}", stmt);
        //    }

        fn debug() -> App {
            match rusqlite::Connection::open("./data/debug.db") {
                Ok(connection) => {
                    let app = App {
                        cnn: connection,
                        fm: FinanceManager::new(),
                    };
                    println!("Banco de dados debug acessado com sucesso");

                    app.init_database();
                    app._insert_debug_data();

                    return app;
                }
                Err(_) => panic!("Banco de dados não encontrado"),
            }
        }

        fn init_database(&self) {
            self.cnn
                .execute("CREATE TABLE Persons (name TEXT PRIMARY KEY)", [])
                .unwrap();
            self.cnn
                .execute(
                    "CREATE TABLE Accounts (id INTEGER PRIMARY KEY, title TEXT NOT NULL)",
                    [],
                )
                .unwrap();
            self.cnn
                .execute("CREATE TABLE PersonsAccounts (
                    person_name TEXT NOT NULL,
                    account_id INTEGER NOT NULL,
                    PRIMARY KEY (person_name, account_id),
                    FOREIGN KEY (person_name) REFERENCES Persons(name)
                        ON DELETE CASCADE
                        ON UPDATE CASCADE,
                    FOREIGN KEY (account_id) REFERENCES Accounts(id)
                        ON DELETE CASCADE
                        ON UPDATE CASCADE)", []).unwrap();
        }

        fn _insert_debug_data(&self) {
            self.cnn
                .execute(
                    "INSERT INTO Persons (name) VALUES ('Roberto'), ('Alma'), ('Mariana')",
                    [],
                )
                .unwrap();

            self.cnn.execute("INSERT INTO Accounts (id, title) VALUES (1, 'carteira'), (2, 'nubank'), (3, 'caixa')", []);

            self.cnn.execute("INSERT INTO PersonsAccounts (person_name, account_id)  VALUES 
            ('Roberto', 1),
            ('Roberto', 2)", []);
        }
    }

    let app = App::debug();
    // app._insert_debug_data();

    // app.load_persons();
}
