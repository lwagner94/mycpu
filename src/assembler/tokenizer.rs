use std::io::BufRead;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Line {
    pub line_number: usize,
    pub text: String,
}

#[derive(Debug)]
pub struct Token {
    pub token: String,
    pub position: usize,
}

#[derive(Debug)]
pub struct TokenizedLine {
    pub line: Line,
    pub tokens: Vec<Token>,
}

fn strip_comments(line: String) -> String {
    match line.find("//") {
        Some(location) => line[..location].into(),
        None => line,
    }
}

fn trim(line: &str) -> String {
    line.trim().into()
}

fn split_lines(reader: &mut BufRead) -> Vec<Line> {
    let mut result = Vec::new();

    for (mut line_number, text_result) in reader.lines().enumerate() {
        let text = text_result.unwrap();

        line_number += 1;

        result.push(Line { line_number, text })
    }

    result
}

fn tokenize_line(text: &str) -> Vec<Token> {
    let replaced = text.replace(",", " ");
    replaced
        .split_whitespace()
        .map(|s| Token {
            token: s.into(),
            position: 0,
        })
        .collect()
}

pub fn tokenize(reader: &mut BufRead) -> Vec<TokenizedLine> {
    let lines = split_lines(reader);

    let mut tokenized_lines = Vec::new();

    for line in lines {
        let text = trim(&strip_comments(line.text.clone()));
        if text.is_empty() {
            continue;
        }
        let tokens: Vec<Token> = tokenize_line(&text);

        tokenized_lines.push(TokenizedLine { line, tokens })
    }

    tokenized_lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_strip_comments_no_comment() {
        let s = "foobar";
        assert_eq!(&strip_comments(s.into()), s);
    }

    #[test]
    fn test_strip_comments_comment() {
        let s = "foobar // test";
        assert_eq!(&strip_comments(s.into()), "foobar ");
    }

    #[test]
    fn test_trim_no_change() {
        let s = "foobar";
        assert_eq!(&trim(s.into()), s);
    }

    #[test]
    fn test_trim() {
        let s = "  foobar   \t";
        assert_eq!(&trim(s.into()), "foobar");
    }

    #[test]
    fn test_split_lines_empty() {
        let mut s = Cursor::new("abc\n\nghi");
        let res = split_lines(&mut s);

        assert_eq!(
            res,
            vec![
                Line {
                    line_number: 1,
                    text: "abc".into()
                },
                Line {
                    line_number: 2,
                    text: "".into()
                },
                Line {
                    line_number: 3,
                    text: "ghi".into()
                }
            ]
        );
    }
}
