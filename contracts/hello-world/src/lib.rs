// Library
// Add Book
// Remove Book
// Find Book
// List Book
// Count Book

// Storing data - Getting Stored data - Data Type - Business Logic

#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol, Vec};

#[derive(Debug, Clone)]
#[contracttype]
pub struct Book {
    pub title: Symbol,
    pub author: Symbol,
    pub year: u32,
}

#[derive(Clone)]
#[contracttype]
pub struct Library {
    books: Vec<Book>,
}

const LIBRARY_KEY: Symbol = symbol_short!("LIBRARY");

pub trait LibraryTrait {
    fn initialize(env: Env);
    fn add_book(env: Env, title: Symbol, author: Symbol, year: u32);
    fn remove_book(env: Env, title: Symbol);
    fn find_book(env: Env, title: Symbol) -> Option<Book>;
    fn list_books(env: Env) -> Vec<Book>;
    fn count_books(env: Env) -> u32;
}

#[contract]
pub struct LibraryContract;

#[contractimpl]
impl LibraryTrait for LibraryContract {
    fn initialize(env: Env) {
        // todo!()
        let library = Library {
            books: Vec::new(&env),
        };

        // add the new book into the storage.
        env.storage().instance().set(&LIBRARY_KEY, &library);
    }

    fn add_book(env: Env, author: Symbol, title: Symbol, year: u32) {
        // todo!()
        let mut library: Library =
            env.storage()
                .instance()
                .get(&LIBRARY_KEY)
                .unwrap_or_else(|| Library {
                    books: Vec::new(&env),
                });

        library.books.push_back(Book {
            title,
            author,
            year,
        });

        env.storage().instance().set(&LIBRARY_KEY, &library);
    }

    fn remove_book(env: Env, title: Symbol) {
        // todo!()
        let mut library: Library = env.storage().instance().get(&LIBRARY_KEY).unwrap_or_else( || Library { books: Vec::new(&env)});

        let index = library.books.iter().position( |book | book.title == title);

        if let Some(i) = index {
            library.books.remove(i as u32);
            
            env.storage().instance().set(&LIBRARY_KEY, &library);
        }
    }

    fn find_book(env: Env, title: Symbol) -> Option<Book> {
        // todo!()
        let library: Library = env.storage().instance().get(&LIBRARY_KEY).unwrap_or_else(|| Library { books: Vec::new(&env)});

        let the_book = library.books.into_iter().find(|book|book.title == title);

        return the_book 

    }

    fn list_books(env: Env) -> Vec<Book> {
        // todo!()
        let library: Library = env.storage().instance().get(&LIBRARY_KEY).unwrap_or_else(|| Library { books:  Vec::new(&env) });

        library.books
    }
    fn count_books(env: Env) -> u32 {
        todo!()
    }
}

mod test;
