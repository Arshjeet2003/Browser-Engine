use std::iter::Peekable;
use std::str::Chars;

use crate::dom::{AttrMap, ElementData, Node, NodeType};

pub struct HtmlParser<'a> {
    chars: Peekable<Chars<'a>>,
    node_q: Vec<String>,
}

impl<'a> HtmlParser<'a> {
    pub fn new(full_html: &str) -> HtmlParser {
        HtmlParser {
            chars: full_html.chars().peekable(),
            node_q: Vec::new(),
        }
    }

/* Step by step process 
    Example
    Input HTML

        <div class="container">
            <p>Hello, <b>world</b>!</p>
        </div>

    parse_nodes() -> '<' -> parse_node() -> div
    -> parse_attributes() -> Node(Element: div, AttrMap: {"class": "container"})

    -> children (div): 
    parse_nodes() -> '<' -> parse_node() -> p 
    -> parse_attributes() -> Node(Element: p, AttrMap: {})

    -> children (p):
    parse_nodes() -> "Hello" (nodes: {"Hello"} ; node_q: {})
    -> '<' -> parse_node() -> b
    -> parse_attributes() -> Node(Element: b, AttrMap: {})

    -> children (b):
    parse_nodes() -> "World" (nodes: {"World"} ; node_q: {})
    -> '</' -> node_q.push() (nodes: {"World"} ; node_q: {b}) 
    
    b
    |--"World"
    
    return to -> children (p):
    -> insert_index = 1 ->  checked if assumed tag is equal to tag in node_q top pos (both are b)
    -> nodes: {"Hello", b} (pushed to children of p)
    -> '</' -> node_q.push() (nodes: {"Hello", b} ; node_q: {p})
    
    p
    |--"Hello"
    |-- b
        |--"World"
    
    return to -> children (div):
    -> insert_index = 0 -> checked if assumed tag is equal to tag in node_q top pos (both are p)
    -> nodes: {p} (pushed to children of div)
    -> '</' -> node_q.push() (nodes: {p} ; node_q: {div}) 

    div
    |-- p
        |--"Hello"
        |-- b
            |--"World"

    if at any place assumed_tag does not match node_q then we directly append nodes children instead
    of that node and keep the assumed tag still in node_q as later on it can match.

    For more details:
    Example: Handling Unexpected Closing Tags
    Input HTML

    <div>
     <p>Hello, <b>world</p>!</b>
    </div>

    Parsing Steps

     The parser encounters <div> and correctly opens it.
     It finds <p> and expects a matching </p> tag.
     It encounters <b>world</p>!.
         It expects </b> but finds </p>.
         The assumed closing tag is </b>, but the actual one is </p>.
     The parser detects the mismatch:
         Instead of inserting the <b> node, it moves its children ("world") into nodes.
         It restores the assumed tag (b) for later correction.
     The p node is inserted.

    Final Node Tree

    Node(Element: div)
    │
    ├── Node(Element: p)
    │   ├── Node(Text: "Hello, ")
    │   ├── Node(Text: "world")
    │   ├── Node(Element: b)
    │
*/


    pub fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        /* first check if '<' -> then either in order as checked
        * closing tag (</div>)
        * comment (<!-- comment -->)
        * opening tag (<div>) -> recursively parse all children
           
        else -> text node (hello) in <p>hello</p>
        */

        while self.chars.peek().is_some() {
            self.consume_while(char::is_whitespace);
            if self.chars.peek().map_or(false, |c| *c == '<') {
                self.chars.next();
                if self.chars.peek().map_or(false, |c| *c == '/') {
                    self.chars.next();
                    self.consume_while(char::is_whitespace);

                    let close_tag_name = self.consume_while(is_valid_tag_name);
                    self.consume_while(|x| x != '>');
                    self.chars.next();
                    self.node_q.push(close_tag_name);
                    break;
                } else if self.chars.peek().map_or(false, |c| *c == '!') {
                    self.chars.next();
                    nodes.push(self.parse_comment_node());
                } else {
                    let mut node = self.parse_node();
                    let insert_index = nodes.len();

                    
                    match node.node_type {
                        NodeType::Element(ref e) => {
                            if self.node_q.len() > 0 {
                                let assumed_tag = self.node_q.remove(0);

                                if e.tag_name != assumed_tag {
                                    nodes.append(&mut node.children);
                                    self.node_q.insert(0, assumed_tag);
                                }
                            }
                        }
                        _ => {}
                    }
                    nodes.insert(insert_index, node);
                }
            } else {
                nodes.push(self.parse_text_node());
            }
        }
        nodes
    }

/*
        The parse_node() function is responsible for parsing an HTML element and its attributes, then recursively parsing its child nodes.
    How It Works

        Extract the tag name
            Uses consume_while(is_valid_tag_name) to read the tag name.
        Parse attributes
            Calls parse_attributes() to extract any attributes associated with the tag.
        Create an ElementData instance
            Uses the parsed tag name and attributes.
        Recursively parse child nodes
            Calls parse_nodes() to parse all nested elements or text within this element.
        Return a Node
            Constructs and returns a Node representing this HTML element.

    Example
    Input HTML

    <div class="container">
        <p>Hello, <b>world</b>!</p>
    </div>

    Parsing Steps

        The parser encounters <div class="container">.
            Tag name: "div"
            Attributes: { "class": "container" }
        It recursively calls parse_nodes() to parse children.
            Encounters <p>.
                Tag name: "p"
                Attributes: {} (none) as nothing before '>' of '<p>'
                Parses its children:
                    "Hello, " (text node) (child 1)

                    Tag name: "b" (child 2)
                    Attributes: {} (none)
                    Parses its children:
                    "world" (text node)

        It returns a structured representation.

    Output: Parsed Node Tree

    Node(Element: div)
    │
    ├── Node(Element: p)
    │   ├── Node(Text: "Hello, ")
    │   ├── Node(Element: b)
    │   │   ├── Node(Text: "world")
    │
*/

    fn parse_node(&mut self) -> Node {
        let tagname = self.consume_while(is_valid_tag_name);
        let attributes = self.parse_attributes();

        let elem = ElementData::new(tagname, attributes);
        let children = self.parse_nodes();
        Node::new(NodeType::Element(elem), children)
    }

    fn parse_text_node(&mut self) -> Node {
        let mut text_content = String::new();

        while self.chars.peek().map_or(false, |c| *c != '<') {
            let whitespace = self.consume_while(char::is_whitespace);
            if whitespace.len() > 0 {
                text_content.push(' ');
            }
            let text_part = self.consume_while(|x| !x.is_whitespace() && x != '<');
            text_content.push_str(&text_part);
        }
        Node::new(NodeType::Text(text_content), Vec::new())
    }

    fn parse_comment_node(&mut self) -> Node {
        let mut comment_content = String::new();

        if self.chars.peek().map_or(false, |c| *c == '-') {
            self.chars.next();
            if self.chars.peek().map_or(false, |c| *c == '-') {
                self.chars.next();
            } else {
                self.consume_while(|c| c != '>');
                return Node::new(NodeType::Comment(comment_content), Vec::new());
            }
        } else {
            self.consume_while(|c| c != '>');
            return Node::new(NodeType::Comment(comment_content), Vec::new());
        }
        if self.chars.peek().map_or(false, |c| *c == '>') {
            self.chars.next();
            return Node::new(NodeType::Comment(comment_content), Vec::new());
        }

        if self.chars.peek().map_or(false, |c| *c == '-') {
            self.chars.next();
            if self.chars.peek().map_or(false, |c| *c == '>') {
                self.chars.next();
                return Node::new(NodeType::Comment(comment_content), Vec::new());
            } else {
                comment_content.push('-');
            }
        }

        while self.chars.peek().is_some() {
            comment_content.push_str(&self.consume_while(|c| c != '<' && c != '-'));
            if self.chars.peek().map_or(false, |c| *c == '<') {
                self.chars.next();
                if self.chars.peek().map_or(false, |c| *c == '!') {
                    self.chars.next();
                    if self.chars.peek().map_or(false, |c| *c == '-') {
                        self.chars.next();
                        if self.chars.peek().map_or(false, |c| *c == '-') {
                            self.consume_while(|c| c != '>');

                            return Node::new(NodeType::Comment(String::from("")), Vec::new());
                        } else {
                            comment_content.push_str("<!-");
                        }
                    } else if self.chars.peek().map_or(false, |c| *c == ' ') {
                        self.chars.next();
                        if self.chars.peek().map_or(false, |c| *c == '-') {
                            self.chars.next();
                            if self.chars.peek().map_or(false, |c| *c == '-') {
                                self.chars.next();
                                if self.chars.peek().map_or(false, |c| *c == '-') {
                                    self.chars.next();
                                    if self.chars.peek().map_or(false, |c| *c == '>') {
                                        self.chars.next();
                                        return Node::new(
                                            NodeType::Comment(String::from("")),
                                            Vec::new(),
                                        );
                                    } else {
                                        comment_content.push_str("<! --");
                                    }
                                } else {
                                    comment_content.push_str("<! -");
                                }
                            } else {
                                comment_content.push_str("<! ");
                            }
                        }
                    } else {
                        comment_content.push_str("<!");
                    }
                } else {
                    comment_content.push('<');
                }
            } else if self.chars.peek().map_or(false, |c| *c == '-') {
                self.chars.next();
                if self.chars.peek().map_or(false, |c| *c == '-') {
                    self.chars.next();
                    if self.chars.peek().map_or(false, |c| *c == '>') {
                        self.chars.next();
                        break;
                    } else {
                        comment_content.push_str("--");
                    }
                } else {
                    comment_content.push('-');
                }
            }
        }

        Node::new(NodeType::Comment(comment_content), Vec::new())
    }

    fn parse_attributes(&mut self) -> AttrMap {
        let mut attributes = AttrMap::new();

        while self.chars.peek().map_or(false, |c| *c != '>') {
            self.consume_while(char::is_whitespace);
            let name = self.consume_while(|c| is_valid_attr_name(c)).to_lowercase();
            self.consume_while(char::is_whitespace);

            let value = if self.chars.peek().map_or(false, |c| *c == '=') {
                self.chars.next();
                self.consume_while(char::is_whitespace);
                let s = self.parse_attr_value();
                self.consume_while(|c| !c.is_whitespace() && c != '>');
                self.consume_while(char::is_whitespace);
                s
            } else {
                "".to_string()
            };
            attributes.insert(name, value);
        }
        self.chars.next();
        attributes
    }

    fn parse_attr_value(&mut self) -> String {
        self.consume_while(char::is_whitespace);
        let result = match self.chars.peek() {
            Some(&c) if c == '"' || c == '\'' => {
                self.chars.next();
                let ret = self.consume_while(|x| x != c);
                self.chars.next();
                ret
            }
            _ => self.consume_while(is_valid_attr_value),
        };
        result
    }

    fn consume_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while self.chars.peek().map_or(false, |c| condition(*c)) {
            result.push(self.chars.next().unwrap());
        }
        result
    }
}
// .is_digit(36) allows: Alphanumeric characters (a-z, A-Z, 0-9).
fn is_valid_tag_name(ch: char) -> bool {
    ch.is_digit(36)
}

/* Checks if a character can be part of an attribute name.
Valid: class, id, data-value
Invalid: Contains =, ", ', whitespace
*/
fn is_valid_attr_name(c: char) -> bool {
    !is_excluded_name(c) && !is_control(c)
}

/* Checks if a character is a control character, which is not allowed.
Control characters are non-printable characters like:
     \n (newline)
     \t (tab)
*/
fn is_control(ch: char) -> bool {
    match ch {
        '\u{007F}' => true,
        c if c >= '\u{0000}' && c <= '\u{001F}' => true,
        c if c >= '\u{0080}' && c <= '\u{009F}' => true,
        _ => false,
    }
}

// Defines characters that are NOT allowed in attribute names.
fn is_excluded_name(c: char) -> bool {
    match c {
        ' ' | '"' | '\'' | '>' | '/' | '=' => true,
        _ => false,
    }
}

/* Defines valid characters for attribute values.
    Valid: abc123
    Invalid: ", ', =, <, >.
*/
fn is_valid_attr_value(c: char) -> bool {
    match c {
        ' ' | '"' | '\'' | '=' | '<' | '>' | '`' => false,
        _ => true,
    }
}
