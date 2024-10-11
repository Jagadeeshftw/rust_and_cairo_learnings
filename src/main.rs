#[derive(Debug)]     // Its kind of importing the Debug traits.
                     // Traits contains collection of functions.

struct Deck {
    cards: Vec<String>
}

fn main() {

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
    println!("Heres your deck: {:#?}", deck); // here {:?} itself will work
                                              // adding {:#?} will print the items
                                              // in the nicely formatted lines
                                              
    
}
