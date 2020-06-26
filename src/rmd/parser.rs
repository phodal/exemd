pub struct Rmd {
    text: String
}

impl Rmd {
    pub fn new(text: String) -> Rmd {
        Rmd {
            text
        }
    }

    pub fn parse(&mut self) {
        println!("start parse")
    }
}