#![allow(dead_code, unused_variables, unused_imports)]
use rusqlite::types::ToSql;
use rusqlite::{params, Connection, Result};

pub const NO_PARAMS: &[&dyn ToSql] = &[];
#[derive(Debug)]
pub struct Bank {
    id: i32,
    name: String,
    age: i32,
    address: String,
    balance: i32,
    account_type: String,
    phone: i32,
}

impl Bank {
    fn new(
        id: i32,
        name: String,
        age: i32,
        address: String,
        balance: i32,
        account_type: String,
        phone: i32,
    ) -> Bank {
        Bank {
            id: id,
            name: name,
            age: age,
            address: address,
            balance: balance,
            account_type: account_type,
            phone: phone,
        }
    }
}

pub fn create_database() -> Result<()> {
    let conn = Connection::open("back.db")?;
    conn.execute_batch(
        "BEGIN;
        create table if not exists bank (id INTEGER PRIMARY KEY, name text, age int, address text, balance int, account_type text, mobile_number int);
        create table if not exists staff (name text, pass text,salary int, position text);
        create table if not exists admin (name text, pass text);
        COMMIT;",
        
    )?;
    let admin: Result<String> = conn.query_row_and_then(
        "SELECT name FROM admin WHERE name='admin'",
        NO_PARAMS,
        |row| row.get(0),
    );

    match admin {
        Ok(st) => {
            println!("{}", st);
        }
        Err(e) => {
            println!("{}", e);
            conn.execute("insert into admin values('admin','123')", NO_PARAMS)?;
        }
    }

    // println!("{}", admin);

    Ok(())
}

pub fn check_auth(name: String, password: String) -> bool {
    let conn = Connection::open("back.db");
    match conn {
        Ok(con) => {
            let admin: Result<String> = con.query_row_and_then(
                "SELECT pass FROM admin WHERE name=?",
                params![name],
                |row| row.get(0),
            );

            match admin {
                Ok(st) => {
                    println!("{}", st);
                    if st == password {
                        true
                    } else {
                        false
                    }
                }
                Err(_) => false,
            }
        }
        _ => false,
    }
}
