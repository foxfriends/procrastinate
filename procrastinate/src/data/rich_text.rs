use serde::{Deserialize, Serialize};
use url::Url;

use super::Id;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Inline {
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
    BoldItalic(Vec<Inline>),
    Link(Url, Box<Inline>),
    UserTag(Id),
    ChannelTag(Id),
    MessageTag(Id),
    Emoji(String),
    Code(Option<String>, String),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Block {
    Paragraph(Vec<Inline>),
    CodeBlock(Option<String>, String),
    Quote(Vec<Inline>),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RichText(Vec<Block>);
