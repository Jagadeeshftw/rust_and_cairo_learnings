// `cargo new <folder-name` will initialize new cargo project.
// `cargo run` command will run the program

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

    let deck = Deck { cards};      
     return deck;
    }

}

fn main() {
    let deck = Deck::new();
    println!("Heres your deck: {:#?}", deck); // here {:?} itself will work
                                              // adding {:#?} will print the items
                                              // in the nicely formatted lines
                    
    
}
