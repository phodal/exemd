use crate::rmd::command::Command;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag};

pub struct Rmd {
    text: String,
}

impl Rmd {
    pub fn new(text: String) -> Rmd {
        Rmd { text }
    }

    pub fn parse(&mut self) -> Vec<Command> {
        let mut parser = create_markdown_parser(&self.text).into_offset_iter();
        let mut commands = vec![];
        let mut current_command = Command::new(1);
        let mut text = "".to_string();

        while let Some((event, _offset)) = parser.next() {
            match event {
                Event::Start(Tag::CodeBlock(info)) => {
                    match info {
                        CodeBlockKind::Fenced(lang_code) => {
                            current_command.script.executor = lang_code.to_string();
                        }
                        CodeBlockKind::Indented => {}
                    }

                    text = "".to_string();
                }
                Event::End(Tag::CodeBlock(info)) => {
                    match info {
                        CodeBlockKind::Fenced(_lang_code) => {
                            current_command.script.source = text.to_string();

                            commands.push(current_command.build());
                            current_command = Command::new(0);
                        }
                        CodeBlockKind::Indented => {}
                    }
                }
                Event::Text(body) => {
                    text += &body.to_string();
                }
                Event::Code(inline_code) => {
                    text += &format!("`{}`", inline_code);
                }
                _ => (),
            }
        }

        commands
    }
}

fn create_markdown_parser(content: &str) -> Parser {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    Parser::new_ext(&content, options)
}

#[cfg(test)]
const TEST_MASKFILE: &str = r#"
# Document Title

This is an example maskfile for the tests below.

## serve (port)

> Serve the app on the `port`

~~~bash
echo "Serving on port $port"
~~~


## node (name)

> An example node script

Valid lang codes: js, javascript

```js
const { name } = process.env;
console.log(`Hello, ${name}!`);
```


## no_script

This command has no source/script.
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_success_parse_len() {
        let mut rmd = Rmd::new(TEST_MASKFILE.to_string());
        let tree = rmd.parse();
        assert_eq!(2, tree.len());
    }

    #[test]
    fn should_success_parse_multiple() {
        let mut rmd = Rmd::new(TEST_MASKFILE.to_string());
        let tree = rmd.parse();
        assert_eq!("bash", tree[0].script.executor);
        assert_eq!(
            "echo \"Serving on port $port\"
",
            tree[0].script.source
        );
        assert_eq!("js", tree[1].script.executor);
        assert_eq!(
            "const { name } = process.env;
console.log(`Hello, ${name}!`);
",
            tree[1].script.source
        );
    }
}
