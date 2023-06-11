use super::{cursor::Cursor, env::Line, k::*, state::State};

pub fn parse_bytes(bytes: &Vec<u8>) -> Vec<Line> {
    let mut cursor = Cursor::new();

    for &byte in bytes {
        match byte {
            K_LF => {
                cursor.commit();
            }

            K_EQUAL => match cursor.state {
                State::Key => {
                    cursor.state = State::Value;
                }
                State::Comment => {
                    cursor.comment.value.push('=');
                }
                State::Value => {
                    cursor.variable.value.push('=');
                }
                State::Margin => {
                    cursor.push_margin_to_value();

                    cursor.state = State::Value;
                    cursor.variable.value.push('=');
                }
                State::Indent => {}
                State::None => {}
                State::Quote => {
                    cursor.variable.value.push('=');
                }
            },

            K_SPACE => match cursor.state {
                State::Margin => {
                    cursor.margin.size += 1;
                }
                State::Key => {}
                State::Value => {
                    cursor.state = State::Margin;
                    cursor.margin.size += 1;
                }
                State::Comment => {
                    cursor.comment.value.push(' ');
                }
                State::None => {
                    cursor.state = State::Indent;
                    cursor.indent.size += 1;
                }
                State::Indent => {
                    cursor.indent.size += 1;
                }
                State::Quote => {
                    cursor.variable.value.push(' ');
                }
            },

            K_HASH => match cursor.state {
                State::Margin => {
                    cursor.state = State::Comment;
                }
                State::Key => {}
                State::Value => {
                    cursor.variable.value.push('#');
                }
                State::Comment => {
                    cursor.comment.value.push('#');
                }
                State::Indent => {
                    cursor.state = State::Comment;
                }
                State::None => {
                    cursor.state = State::Comment;
                }
                State::Quote => {
                    cursor.variable.value.push('#');
                }
            },

            K_DOUBLE_QUOTE => match cursor.state {
                State::Key => {}
                State::Value => {
                    cursor.state = State::Quote;
                    cursor.variable.value.push('\"');
                }
                State::Comment => {}
                State::Margin => {}
                State::Indent => {}
                State::None => {}
                State::Quote => {
                    cursor.state = State::Margin;
                    cursor.variable.value.push('\"');
                }
            },

            _ => match cursor.state {
                State::Key => {
                    cursor.variable.key.push(byte as char);
                }
                State::Value => {
                    cursor.variable.value.push(byte as char);
                }
                State::Comment => {
                    cursor.comment.value.push(byte as char);
                }
                State::Margin => {
                    cursor.push_margin_to_value();

                    cursor.state = State::Value;
                    cursor.variable.value.push(byte as char);
                }
                State::Indent => {
                    cursor.state = State::Key;
                    cursor.variable.key.push(byte as char);
                }
                State::None => {
                    cursor.state = State::Key;
                    cursor.variable.key.push(byte as char);
                }
                State::Quote => {
                    cursor.variable.value.push(byte as char);
                }
            },
        }
    }

    return cursor.body;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::env::*;

    #[test]
    fn var() {
        let input = String::from("ONE=1\n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: None,
            variable: Some(Variable {
                key: String::from("ONE"),
                value: String::from("1"),
            }),
            margin: None,
            comment: None,
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn indent_var() {
        let input = String::from("  ONE=1\n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: Some(Indent { size: 2 }),
            variable: Some(Variable {
                key: String::from("ONE"),
                value: String::from("1"),
            }),
            margin: None,
            comment: None,
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn indent_var_margin() {
        let input = String::from("  ONE=1  \n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: Some(Indent { size: 2 }),
            variable: Some(Variable {
                key: String::from("ONE"),
                value: String::from("1"),
            }),
            margin: Some(Margin { size: 2 }),
            comment: None,
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn comment() {
        let input = String::from("# Comment\n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: None,
            variable: None,
            margin: None,
            comment: Some(Comment {
                value: String::from(" Comment"),
            }),
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn comment_with_hashes() {
        let input = String::from("### Comment ###\n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: None,
            variable: None,
            margin: None,
            comment: Some(Comment {
                value: String::from("## Comment ###"),
            }),
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn comment_var() {
        let input = String::from("# API_URL=http://localhost:3000/\n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: None,
            variable: None,
            margin: None,
            comment: Some(Comment {
                value: String::from(" API_URL=http://localhost:3000/"),
            }),
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn var_url_with_hash() {
        let input = String::from("API_URL=http://localhost:3000?a=1#app\n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: None,
            variable: Some(Variable {
                key: String::from("API_URL"),
                value: String::from("http://localhost:3000?a=1#app"),
            }),
            margin: None,
            comment: None,
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn indent_var_margin_comment() {
        let input = String::from("  ONE=Woo  # This is Woo\n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: Some(Indent { size: 2 }),
            variable: Some(Variable {
                key: String::from("ONE"),
                value: String::from("Woo"),
            }),
            margin: Some(Margin { size: 2 }),
            comment: Some(Comment {
                value: String::from(" This is Woo"),
            }),
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn var_with_spaces() {
        let input = String::from("SECRET=Maybe some secret\n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: None,
            variable: Some(Variable {
                key: String::from("SECRET"),
                value: String::from("Maybe some secret"),
            }),
            margin: None,
            comment: None,
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn var_with_spaces_margin_comment() {
        let input = String::from("SECRET=Maybe some secret # Comment\n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: None,
            variable: Some(Variable {
                key: String::from("SECRET"),
                value: String::from("Maybe some secret"),
            }),
            margin: Some(Margin { size: 1 }),
            comment: Some(Comment {
                value: String::from(" Comment"),
            }),
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn var_margin_equal() {
        let input = String::from("SECRET=Maybe some secret = off # No secrets for you\n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: None,
            variable: Some(Variable {
                key: String::from("SECRET"),
                value: String::from("Maybe some secret = off"),
            }),
            margin: Some(Margin { size: 1 }),
            comment: Some(Comment {
                value: String::from(" No secrets for you"),
            }),
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn var_with_quotes() {
        let input = String::from("API_URL=\"http://localhost:3000/\"\n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: None,
            variable: Some(Variable {
                key: String::from("API_URL"),
                value: String::from("\"http://localhost:3000/\""),
            }),
            margin: None,
            comment: None,
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn var_with_quotes_comment() {
        let input = String::from("API_URL=\"http://localhost:3000/\" #Comment\n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: None,
            variable: Some(Variable {
                key: String::from("API_URL"),
                value: String::from("\"http://localhost:3000/\""),
            }),
            margin: Some(Margin { size: 1 }),
            comment: Some(Comment {
                value: String::from("Comment"),
            }),
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn var_with_quotes_with_comment_with_var() {
        let input = String::from("API_URL=\"http://localhost:3000/ # ONE=1\"\n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: None,
            variable: Some(Variable {
                key: String::from("API_URL"),
                value: String::from("\"http://localhost:3000/ # ONE=1\""),
            }),
            margin: None,
            comment: None,
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn indent_var_with_quotes_with_comment_with_var_margin_comment() {
        let input = String::from("    API_URL=\"http://localhost:3000/ # ONE=1\"    # Yay! \n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: Some(Indent { size: 4 }),
            variable: Some(Variable {
                key: String::from("API_URL"),
                value: String::from("\"http://localhost:3000/ # ONE=1\""),
            }),
            margin: Some(Margin { size: 4 }),
            comment: Some(Comment {
                value: String::from(" Yay! "),
            }),
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn indent_comment() {
        let input = String::from("  # :P\n");
        let actual = parse_bytes(&input.into_bytes());
        let expected = vec![Line {
            line: 1,
            indent: Some(Indent { size: 2 }),
            variable: None,
            margin: None,
            comment: Some(Comment {
                value: String::from(" :P"),
            }),
        }];
        assert_eq!(actual, expected);
    }
}
