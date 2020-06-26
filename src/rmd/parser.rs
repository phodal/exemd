use pulldown_cmark::{
    Event::{Code, End, Html, Start, Text},
    Options, Parser, Tag,
};

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
        let parser = create_markdown_parser(&self.text);
        let mut text = "".to_string();

        for event in parser {
            match event {
                Start(tag) => {

                }
                End(tag) => {

                }
                Text(body) => {

                }
                Html(html) => {

                }
                Code(inline_code) => {
                    text += &format!("`{}`", inline_code);
                    println!("{}", text)
                }
                _ => (),
            }
        }
    }
}

fn create_markdown_parser(content: &String) -> Parser {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&content, options);
    parser
}