use pulldown_cmark::{Event::{Code, End, Html, Start, Text}, Options, Parser, Tag, CodeBlockKind};
use crate::rmd::command::Command;

pub struct Rmd {
    text: String
}

impl Rmd {
    pub fn new(text: String) -> Rmd {
        Rmd {
            text
        }
    }

    pub fn parse(&mut self) -> Vec<Command> {
        let parser = create_markdown_parser(&self.text);
        let mut commands = vec![];
        let mut current_command = Command::new(1);
        let mut text = "".to_string();

        for event in parser {
            match event {
                Start(tag) => {
                    match tag {
                        #[cfg(not(windows))]
                        Tag::CodeBlock(info) => {
                            match info {
                                CodeBlockKind::Fenced(lang_code) => {
                                    if lang_code.to_string() != String::from("powershell")
                                        && lang_code.to_string() != String::from("batch")
                                        && lang_code.to_string() != String::from("cmd")
                                    {
                                        current_command.script.executor = lang_code.to_string();
                                    }
                                }
                                CodeBlockKind::Indented => {}
                            }
                        }
                        _ => (),
                    }

                    text = "".to_string();
                }
                End(tag) => {
                    match tag {
                        #[cfg(not(windows))]
                        Tag::CodeBlock(info) => {
                            match info {
                                CodeBlockKind::Fenced(lang_code) => {
                                    if lang_code.to_string() != String::from("powershell")
                                        && lang_code.to_string() != String::from("batch")
                                        && lang_code.to_string() != String::from("cmd")
                                    {
                                        current_command.script.source = text.to_string();
                                    }
                                }
                                CodeBlockKind::Indented => {}
                            }
                        }
                        _ => (),
                    }
                }
                Text(body) => {
                    text += &body.to_string();
                }
                Html(html) => {}
                Code(inline_code) => {
                    text += &format!("`{}`", inline_code);
                }
                _ => (),
            }
        }

        commands.push(current_command.build());
        commands
    }
}


fn create_markdown_parser(content: &String) -> Parser {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&content, options);
    parser
}