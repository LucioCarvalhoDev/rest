
use std::{collections::HashMap, io::{self, Write}};

use atoms::atoms::IDWrapper;
use sqlite::Connection;

use crate::{
    atoms::atoms::{Person, PersonID},
    finance_manager::fm::FinanceManager,
};

pub mod atoms;
pub mod finance_manager;

fn main() {
    let _ = std::fs::write("./data/debug.db", "");

    struct DbManager {
        connection: Connection,
        db_path: String
    }

    impl DbManager {
        fn new(pth: &str) -> DbManager {
            let cnn = match sqlite::open(pth) {
                Ok(connection) => {
                    println!("Conectado a {}", pth);
                    connection
                },
                Err(_) => todo!(),
            };

            let mut app = DbManager {
                connection: cnn,
                db_path: pth.to_string()
            };

            return app;
        }

        fn reset_db(&self) {
            match std::fs::write(&self.db_path, "") {
                Ok(_) => {
                    println!("Banco de dados resetado!");
                },
                Err(_) => {todo!()}
            };
            
            self.connection.execute("
            CREATE TABLE Persons (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )").unwrap();

            self.connection.execute("
            CREATE TABLE Accounts (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL
            )").unwrap();

            self.connection.execute("
            CREATE TABLE PersonsAccounts (
                person_id INTEGER NOT NULL,
                account_id INTEGER NOT NULL,
                PRIMARY KEY (person_id, account_id),
                FOREIGN KEY (person_id) REFERENCES Persons(id)
                    ON DELETE CASCADE
                    ON UPDATE CASCADE,
                FOREIGN KEY (account_id) REFERENCES Accounts(id)
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            )").unwrap();
        }

        fn insert_debug_dataset(&self) {
            self.connection.execute("
            INSERT INTO Persons VALUES
            (1, 'Roberto'),
            (2, 'Alma'),
            (3, 'Caliane')
            ").unwrap();
        }

        fn select_persons(&self) -> Vec<Person> {
            let mut res = vec![];

            let mut pre_query = self.connection.prepare("
            SELECT id, name FROM Persons;
            ").unwrap();

            while let Ok(sqlite::State::Row) = pre_query.next() {
                let raw_id= pre_query.read::<i64, _>("id").unwrap();
                let raw_name= pre_query.read::<String, _>("name").unwrap();
                let person = Person { name: raw_name, id: PersonID(raw_id as i32)};
                res.push(person);
            }

            return res;
        }

    }

    struct App {
        db: DbManager,
        fm: FinanceManager,
        changes: Vec<IDWrapper>
    }

    impl App {
        fn new() -> App {
            return App {
                db: DbManager::new("./data/rest.db"),
                fm: FinanceManager::new(),
                changes: vec![]
            }
        }

        fn load_persons(&mut self) -> Vec<PersonID> {      
            let mut persons = self.db.select_persons();
            let mut biggest_id = 0;

            let mut id_list = vec![];
            
            while persons.len()>0 {
                let person = persons.pop().unwrap();

                if (biggest_id<person.id.0) {
                    biggest_id = person.id.0;
                }

                id_list.push(person.id);
                self.fm.persons.insert(person.id, person); 
            }
            self.fm.id_generator._person = biggest_id;
            return id_list;
        }

        fn add_person(&mut self, name: &str) {
            self.changes.push(IDWrapper::WrapperPersonID(self.fm.create_person(name)));
        }

    }

    let mut app = App::new();

    println!("Número de Persons carregas do db: {:?}", app.load_persons());
    app.add_person("Tiago");
    println!("{:?}", app.changes);



}
