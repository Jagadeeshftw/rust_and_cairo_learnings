use std::vec;

#[derive(Debug)]
enum Media {
    Movie { title: String, director: String },
    Book { title: String, author: String },
    AudioBook { title: String },
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::AudioBook { title } => {
                format!("{} Audiobook", title)
            }
            Media::Movie { title, director } => {
                format!("{} directored by {}", title, director)
            }
            Media::Book { title, author } => {
                format!("{} authored by {}", title, author)
            }
        }
    }
}

#[derive(Debug)]
struct Castal {
    items: Vec<Media>,
}

impl Castal {
    fn new() -> Self {
        Castal { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
}
fn main() {
    let audio_book = Media::AudioBook {
        title: String::from("Badass"),
    };
    let movie = Media::Movie {
        title: String::from("Leo"),
        director: String::from("Lokesh"),
    };
    let book = Media::Book {
        title: String::from("Comics"),
        author: String::from("Noobita"),
    };
    println!("{}", audio_book.description());
    println!("{}", book.description());
    println!("{}", movie.description());

    let mut castal = Castal::new();
    castal.add(audio_book);
    castal.add(movie);
    castal.add(book);


    // this contains the output as
    // Some(
    //    AudioBook {
    //        title: "Badass",
    //   },
    //)
    println!("{:#?}", castal.items.get(0));
    
    
    // Unlike above to print only the Audiobook and to get rid of the Some
    // below approach is used
    match castal.items.get(0) {
        Option::Some(value) =>{
            println!("{:#?}", value);
        }
        Option::None =>{
            println!("Out of range");
        }
    }

}