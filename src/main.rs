use clap::{App, Arg};
use pulldown_cmark::{Event, Parser, Tag};
use sixtyfps::Model;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
sixtyfps::include_modules!();

#[derive(Debug, Default)]
struct TextProperties {
    size: i32,
    color: sixtyfps::Color,
}
impl TextProperties {
    fn new(size: i32, color: sixtyfps::Color) -> Self {
        Self { size, color }
    }
}

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("PrevMark")
        .version("0.1.0")
        .author("Jared Moulton <jaredmoulton3@gmail.com")
        .about("A document previewer for markdown files")
        .arg(
            Arg::new("path")
                .about("The path the the markdown file to preview")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .get_matches();
    let path = matches.value_of("path").unwrap(); // unwrapping becuase required

    let md_file = match File::open(path) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        }
        Err(e) => return Err(e),
    };

    let parser = Parser::new(&md_file);

    let main_window = PrevMark::new();
    let main_window_weak = main_window.as_weak();

    let text_elements: Vec<TextElement> = main_window_weak
        .unwrap()
        .get_TextElements()
        .iter()
        .collect();
    let text_model = Rc::new(sixtyfps::VecModel::from(text_elements));
    let mut text_properties = TextProperties::default();

    for event in parser {
        match event {
            Event::Start(tag) => {
                dbg!("Start tag ");
                dbg!(&tag);
                match tag {
                    // use these to set properties on the text
                    // Tag::Paragraph => todo!(),
                    Tag::Heading(level) => match level {
                        1 => {
                            text_properties.size = 32;
                        }
                        2 => {
                            text_properties.size = 24;
                        }
                        3 => {
                            text_properties.size = 19;
                        }
                        4 => {
                            text_properties.size = 16;
                        }
                        5 => {
                            text_properties.size = 13;
                        }
                        6 => {
                            text_properties.size = 11;
                        }
                        _ => (),
                    },
                    // Tag::BlockQuote => todo!(),
                    // Tag::CodeBlock(_) => todo!(),
                    // Tag::List(_) => todo!(),
                    // Tag::Item => todo!(),
                    // Tag::FootnoteDefinition(_) => todo!(),
                    // Tag::Table(_) => todo!(),
                    // Tag::TableHead => todo!(),
                    // Tag::TableRow => todo!(),
                    // Tag::TableCell => todo!(),
                    // Tag::Emphasis => todo!(),
                    // Tag::Strong => todo!(),
                    // Tag::Strikethrough => todo!(),
                    Tag::Link(_, link, _title) => {
                        // I' not sure how to make a clickable link
                        text_model.push(TextElement {
                            size: (text_properties.size),
                            text: (link.to_string().into()),
                        });
                    }
                    // Tag::Image(_, _, _) => todo!(),
                    _ => (),
                }
            }
            Event::End(tag) => {
                match tag {
                    Tag::Heading(_) => {
                        // At the end of a heading tag reset the font size back to 12
                        text_properties.size = 12;
                    }
                    _ => (),
                }
                text_model.push(TextElement {
                    size: (text_properties.size),
                    text: ("\n".into()),
                });
                dbg!("End tag");
                dbg!(tag);
            }
            Event::Text(text) => {
                text_model.push(TextElement {
                    size: (text_properties.size),
                    text: (text.to_string().into()),
                });
                dbg!(text);
            }
            Event::Code(code) => {
                dbg!(code);
            }
            Event::Html(html) => {
                // Now i need to parse the html here... This is complicated
                text_model.push(TextElement {
                    size: (text_properties.size),
                    text: (html.to_string().into()),
                });
                dbg!(html);
            }
            Event::FootnoteReference(note) => {
                dbg!(note);
            }
            Event::SoftBreak => {
                dbg!("soft break ");
            }
            Event::HardBreak => {
                text_model.push(TextElement {
                    size: (text_properties.size),
                    text: ("\n".into()),
                });
                dbg!("hard break ");
            }
            Event::Rule => {
                dbg!("Horizontal ruler ");
            }
            Event::TaskListMarker(_) => {
                dbg!("task list marker ");
            }
        }
    }
    dbg!(&text_properties);
    main_window_weak
        .unwrap()
        .set_TextElements(sixtyfps::ModelHandle::new(text_model));

    main_window.run();
    Ok(())
}