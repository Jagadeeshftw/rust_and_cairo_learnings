/*
// `cargo new <folder-name` will initialize new cargo project.
// `cargo run` command will run the program

use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]     // Its kind of importing the Debug traits.
                     // Traits contains collection of functions.

struct Deck {
    cards: Vec<String>
}

// This below implementation is called as `inherit implementation`
// since this implementation name is same as the above struct
// name, it will inherit from the above struct.
// This inherit implementation is used to add some functions to the
// above struct.
impl Deck {
    // This functions are called as associated functions same as class
    //method and this is tied to the struct definition.
    // kinda contructor you could say.
    // here `-> Self` refers to return the Deck struct as return type.
    fn new() -> Self {  

    let suites = ["Hearts", "Spades", "Diamonds"];
    let values = ["Ace", "Two", "Three"];

    // Variables will be called as bindings in rust and all the
    // bindings will be immutable by default.
    // specifying `mut` will make the bindings as mutable(can able to modify)
    let mut cards = vec![];    

    for suite in suites{
        for value in values {
            let card = format!("{} of {}", value, suite);
            cards.push(card);
        }
    }

    return Deck { cards};      // explicit return
    // Deck { cards}           // implicit return
    // both of this return types will work, in implicit return semicolon
    // will not come at the end.

    }
    
    // its kinda of class method or self method. where self is the 
    // reference pointing to the instance
    fn shuffle(&mut self)
    {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards:usize) -> Vec<String>
    {
    /* 
    split_off
     - input = index_number;
     -return type = return all the values from index_number to the end of the index.
    */
     self.cards.split_off(self.cards.len()- num_cards)
    }

}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    // needs to add error handling for larger inputs
    // which is greater than the total number of cards.
    let mut cards = deck.deal(2);

    println!("Here's your deck: {:#?}", deck);
    println!("Here's your hand: {:#?}", cards); // here {:?} itself will work
                                              // adding {:#?} will print the items
                                              // in the nicely formatted lines
                    
    
}



/*
1. To install crates(packages)

`cargo add <package_name>`
e.g: cargo add rand

website: https://crates.io/

installed crates will be shown in Cargo.toml file

2. Crates.

Each crates contains the root modules and the list of sub modules.

for example: In the rand crate there's one root modules called `thread_rng`
             which can be accessed by
             let Rand = rand::thread_rng();

             and lets say there is one sub module called `seq` and that
             has list of functions. one of the function is `SliceRandom`
             which can be accessed by

             let Rand = rand::seq::SliceRandom();

             But when we use our own modules in that case we need to import it
             `mod <sub_module_name`;
             e.g: mod games;
             let game = games::<function_name>();

`use` Keyword:

`
use rand::thread_rng;

let rand = thread_rng();
`
as you can see above using the `use` keyword, we can directly access the
function. we can do the same thing for submodules as well.

`
use Rand::{thread_rng, random, rngs::OsRng};
let osRng = OsRng();
`
         
*/
*/

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    fn summary(&self) -> String {
        format!("{} has a balance {}", self.holder, self.balance)
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("me"));

    account.deposit(500);
    account.withdraw(250);

    bank.add_account(account);

    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
}
