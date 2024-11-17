// option2

pub mod Content {
    #[derive(Debug)]
    pub enum Media {
        Movie { title: String, director: String },
        Book { title: String, author: String },
        AudioBook { title: String },
    }

    impl Media {
        pub fn description(&self) -> String {
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
    pub struct Castal {
        pub items: Vec<Media>,
    }

    impl Castal {
        pub fn new() -> Self {
            Castal { items: vec![] }
        }

        pub fn add(&mut self, media: Media) {
            self.items.push(media);
        }
    }
}