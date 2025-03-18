#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, String, Symbol, Vec};

#[no_std]
#[contract]
pub struct Contract;

// This is a sample contract. Replace this placeholder with your own contract logic.
// A corresponding test example is available in `test.rs`.
//
// For comprehensive examples, visit <https://github.com/stellar/soroban-examples>.
// The repository includes use cases for the Stellar ecosystem, such as data storage on
// the blockchain, token swaps, liquidity pools, and more.
//
// Refer to the official documentation:
// <https://developers.stellar.org/docs/build/smart-contracts/overview>.



const LIBRARY_KEY: Symbol = symbol_short!("LIBRARY");

pub struct Book{
    pub title: Symbol,
    pub author: Symbol,
    pub year: u32
}

pub trait LibraryTrait {
    fn initialize(env: Env);
    fn add_book(env: Env, title: Symbol, author: Symbol, year: u32);
    fn remove_book(env: Env, title: Symbol);
    fn find_book(env: Env, title: Symbol)-> Option<Book>;
    fn list_books(env: Env)-> Vec<Book>;

}

pub struct Library {
    books: Vec<Book>
}


#[contract]
pub struct LibraryContract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

mod test;
