use super::*;

pub fn open(
    _parent: &Node,
    a: &Option<Token>,
    b: &Option<Token>,
    c: &Option<Token>,
) -> Option<(Link, usize)> {
    match (a, b, c) {
        (Some(Token::Hash((start, end))), Some(Token::Whitespace(..)), _) if (end - start) <= 6 => {
            Some((Node::new(Kind::Heading(end - start), *start), *end))
        }
        _ => None,
    }
}

pub fn consume(node: &mut Node, start: usize, source: &str) -> Option<usize> {
    if let Some(p) = leaf::consume(node, start, source) {
        // Headings cannot be continued onto the next line
        // so we close it immediately.
        node.end = Some(p);
        Some(p)
    } else {
        node.end = Some(start);
        None
    }
}