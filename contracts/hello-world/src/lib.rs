#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Vec};

#[contract]
pub struct LibraryContract;

const LIBRARY_KEY: Symbol = symbol_short!("LIBRARY");

#[derive(Debug, Clone)]
pub struct Book {
    pub title: Symbol,
    pub author: Symbol,
    pub year: u32,
}

mod test;