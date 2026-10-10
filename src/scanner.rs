#[derive(Debug)]
pub struct Scanner<'s> {
    src: &'s str,
    pub tokens: Vec<Token>,
    line: usize,
    pub errors: Vec<LexicalError>,
}

impl<'s> Scanner<'s> {
    pub fn new(src: &'s str) -> Self {
        Self {
            src,
            tokens: vec![],
            line: 1,
            errors: vec![],
        }
    }

    pub fn scan_tokens(&mut self) {
        let mut chars = self.src.chars().peekable();

        while let Some(curr) = chars.next() {
            if curr.is_ascii_whitespace() {
                if curr == '\n' {
                    self.line += 1;
                }
                continue;
            }

            let kind = match curr {
                '(' => TokenKind::LeftParen,
                ')' => TokenKind::RightParen,
                '{' => TokenKind::LeftBrace,
                '!' => {
                    if chars.next_if_eq(&'=').is_some() {
                        TokenKind::BangEqual
                    } else {
                        TokenKind::Bang
                    }
                }
                '=' => {
                    if chars.next_if_eq(&'=').is_some() {
                        TokenKind::EqualEqual
                    } else {
                        TokenKind::Equal
                    }
                }
                '<' => {
                    if chars.next_if_eq(&'=').is_some() {
                        TokenKind::LessEqual
                    } else {
                        TokenKind::Less
                    }
                }
                '>' => {
                    if chars.next_if_eq(&'=').is_some() {
                        TokenKind::GreaterEqual
                    } else {
                        TokenKind::Greater
                    }
                }
                '/' => {
                    if chars.next_if_eq(&'/').is_some() {
                        while chars.next_if(|&c| c != '\n').is_some() {}
                        continue;
                    } else {
                        TokenKind::Slash
                    }
                }
                '"' => {
                    let mut string = String::new();
                    while let Some(c) = chars.next_if(|&c| c != '"') {
                        if c == '\n' {
                            self.line += 1;
                        }
                        string.push(c);
                    }
                    if chars.next_if_eq(&'"').is_some() {
                        TokenKind::String(string)
                    } else {
                        self.add_error(LexicalErrorKind::UnterminatedString);
                        break;
                    }
                }
                n if n.is_ascii_digit() => {
                    let mut number = String::from(n);
                    while let Some(c) = chars.next_if(|&c| c.is_ascii_digit() || c == '.') {
                        number.push(c);
                    }
                    if number.ends_with('.') {
                        self.add_error(LexicalErrorKind::UnexpectedCharacter);
                    }
                    TokenKind::Number(number.parse().unwrap())
                }
                c if c.is_alphabetic() => {
                    let mut identifier = String::from(c);
                    while let Some(c) = chars.next_if(|&c| c.is_alphanumeric() || c == '_') {
                        identifier.push(c);
                    }
                    match identifier.as_str() {
                        "and" => TokenKind::And,
                        "class" => TokenKind::Class,
                        "else" => TokenKind::Else,
                        "false" => TokenKind::False,
                        "fun" => TokenKind::Fun,
                        "for" => TokenKind::For,
                        "if" => TokenKind::If,
                        "nil" => TokenKind::Nil,
                        "or" => TokenKind::Or,
                        "print" => TokenKind::Print,
                        "return" => TokenKind::Return,
                        "super" => TokenKind::Super,
                        "this" => TokenKind::This,
                        "true" => TokenKind::True,
                        "var" => TokenKind::Var,
                        "while" => TokenKind::While,
                        _ => TokenKind::Identifier(identifier),
                    }
                }
                _ => {
                    self.add_error(LexicalErrorKind::UnexpectedCharacter);
                    continue;
                }
            };

            self.add_token(kind);
        }

        self.add_token(TokenKind::Eof);
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token {
            kind,
            line: self.line,
        });
    }

    fn add_error(&mut self, kind: LexicalErrorKind) {
        self.errors.push(LexicalError {
            kind,
            line: self.line,
        });
    }
}

#[derive(PartialEq, Debug)]
pub struct Token {
    kind: TokenKind,
    line: usize,
}

#[derive(PartialEq, Debug)]
enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(PartialEq, Debug)]
pub struct LexicalError {
    kind: LexicalErrorKind,
    line: usize,
}

#[derive(PartialEq, Debug)]
pub enum LexicalErrorKind {
    UnexpectedCharacter,
    UnterminatedString,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_eof_token() {
        let src = "";
        let mut scanner = Scanner::new(src);

        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].kind, TokenKind::Eof);
    }

    #[test]
    fn line_increments_on_newline_char() {
        let src = "\n\n\n";
        let mut scanner = Scanner::new(src);

        scanner.scan_tokens();

        assert_eq!(scanner.line, 4);
    }

    #[test]
    fn ignores_whitespaces() {
        let src = " \r\t\n";
        let mut scanner = Scanner::new(src);

        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].kind, TokenKind::Eof);
    }

    #[test]
    fn ignores_single_line_comments() {
        let src = "// Hello, World!";
        let mut scanner = Scanner::new(src);

        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].kind, TokenKind::Eof);
    }

    #[test]
    fn ignores_multi_line_comments() {
        let src = "
            // Hello, World!
            // Hello, Ferris!
        ";
        let mut scanner = Scanner::new(src);

        scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].kind, TokenKind::Eof);
        assert_eq!(scanner.line, 4);
    }

    #[test]
    fn errors_on_unexpected_chars() {
        let src = "@";
        let mut scanner = Scanner::new(src);

        scanner.scan_tokens();

        assert_eq!(
            scanner.errors[0].kind,
            LexicalErrorKind::UnexpectedCharacter
        );
    }

    #[test]
    fn stores_line_where_error_was_found() {
        let src = "\n\n@";
        let mut scanner = Scanner::new(src);

        scanner.scan_tokens();

        assert_eq!(scanner.errors[0].line, 3);
    }

    #[test]
    fn matches_single_char_tokens() {
        let cases = [
            ("(", TokenKind::LeftParen),
            (")", TokenKind::RightParen),
            ("{", TokenKind::LeftBrace),
            ("/", TokenKind::Slash),
        ];

        for (src, kind) in cases {
            let mut scanner = Scanner::new(src);

            scanner.scan_tokens();

            assert_eq!(scanner.tokens[0].kind, kind);
        }
    }

    mod one_or_two_chars_tokens {
        use super::*;

        #[test]
        fn matches_single_char_tokens() {
            let cases = [
                ("!", TokenKind::Bang),
                ("=", TokenKind::Equal),
                ("<", TokenKind::Less),
                (">", TokenKind::Greater),
            ];

            for (src, kind) in cases {
                let mut scanner = Scanner::new(src);

                scanner.scan_tokens();

                assert_eq!(scanner.tokens[0].kind, kind);
            }
        }

        #[test]
        fn matches_side_by_side_single_char_tokens() {
            let src = "=!<>";
            let mut scanner = Scanner::new(src);

            scanner.scan_tokens();

            assert_eq!(scanner.tokens[0].kind, TokenKind::Equal);
            assert_eq!(scanner.tokens[1].kind, TokenKind::Bang);
            assert_eq!(scanner.tokens[2].kind, TokenKind::Less);
            assert_eq!(scanner.tokens[3].kind, TokenKind::Greater);
        }

        #[test]
        fn matches_two_char_tokens() {
            let cases = [
                ("!=", TokenKind::BangEqual),
                ("==", TokenKind::EqualEqual),
                // ("<", TokenKind::Less),
                // (">", TokenKind::Greater),
            ];

            for (src, kind) in cases {
                let mut scanner = Scanner::new(src);

                scanner.scan_tokens();

                assert_eq!(scanner.tokens[0].kind, kind);
            }
        }
    }

    mod string_literals {
        use super::*;

        #[test]
        fn matches_single_line_string_literals() {
            let src = "\"Hello, World!\"";
            let mut scanner = Scanner::new(src);

            scanner.scan_tokens();

            assert_eq!(
                scanner.tokens[0].kind,
                TokenKind::String("Hello, World!".into())
            );
        }

        #[test]
        fn matches_multi_line_string_literals() {
            let src = "\"Hello, World!\nHello, Ferris!\"";
            let mut scanner = Scanner::new(src);

            scanner.scan_tokens();

            assert_eq!(
                scanner.tokens[0].kind,
                TokenKind::String("Hello, World!\nHello, Ferris!".into())
            );
            assert_eq!(scanner.line, 2);
        }

        #[test]
        fn doesnt_match_unterminated_string_literals() {
            let src = "\"Hello, World!";
            let mut scanner = Scanner::new(src);

            scanner.scan_tokens();

            assert_eq!(scanner.tokens.len(), 1);
            assert_eq!(scanner.tokens[0].kind, TokenKind::Eof);
            assert_eq!(scanner.errors[0].kind, LexicalErrorKind::UnterminatedString);
        }
    }

    mod number_literals {
        use super::*;

        fn cmp_number_literal(a: &TokenKind, b: f64) -> bool {
            matches!(a, TokenKind::Number(a) if (a - b).abs() < f64::EPSILON)
        }

        #[test]
        fn matches_integer_literals() {
            let src = "1234";
            let mut scanner = Scanner::new(src);

            scanner.scan_tokens();

            assert!(cmp_number_literal(&scanner.tokens[0].kind, 1234_f64));
        }

        #[test]
        fn matches_floating_point_literals() {
            let src = "12.34";
            let mut scanner = Scanner::new(src);

            scanner.scan_tokens();

            assert!(cmp_number_literal(&scanner.tokens[0].kind, 12.34));
        }

        #[test]
        fn ignores_leading_decimal_point() {
            let src = ".1234";
            let mut scanner = Scanner::new(src);

            scanner.scan_tokens();

            assert!(cmp_number_literal(&scanner.tokens[0].kind, 1234.));
            assert_eq!(
                scanner.errors[0].kind,
                LexicalErrorKind::UnexpectedCharacter
            );
        }

        #[test]
        fn ignores_trailing_decimal_point() {
            let src = "1234.";
            let mut scanner = Scanner::new(src);

            scanner.scan_tokens();

            assert!(cmp_number_literal(&scanner.tokens[0].kind, 1234.));
            assert_eq!(
                scanner.errors[0].kind,
                LexicalErrorKind::UnexpectedCharacter
            );
        }
    }

    mod keywords {
        use super::*;

        #[test]
        fn matches_keywords() {
            let cases = [
                ("and", TokenKind::And),
                ("class", TokenKind::Class),
                ("else", TokenKind::Else),
                ("false", TokenKind::False),
                ("fun", TokenKind::Fun),
                ("for", TokenKind::For),
                ("if", TokenKind::If),
                ("nil", TokenKind::Nil),
                ("or", TokenKind::Or),
                ("print", TokenKind::Print),
                ("return", TokenKind::Return),
                ("super", TokenKind::Super),
                ("this", TokenKind::This),
                ("true", TokenKind::True),
                ("var", TokenKind::Var),
                ("while", TokenKind::While),
            ];

            for (keyword, kind) in cases {
                let mut scanner = Scanner::new(keyword);

                scanner.scan_tokens();

                assert_eq!(scanner.tokens[0].kind, kind);
            }
        }

        #[test]
        fn matches_identifiers() {
            let src = "identifier";
            let mut scanner = Scanner::new(src);

            scanner.scan_tokens();

            assert_eq!(
                scanner.tokens[0].kind,
                TokenKind::Identifier("identifier".into())
            );
        }

        #[test]
        fn matches_following_maximal_munch_rule() {
            let src = "orchid";
            let mut scanner = Scanner::new(src);

            scanner.scan_tokens();

            assert_eq!(
                scanner.tokens[0].kind,
                TokenKind::Identifier("orchid".into())
            );
        }

        #[test]
        fn matches_identifiers_with_numbers() {
            let src = "identifier1";
            let mut scanner = Scanner::new(src);

            scanner.scan_tokens();

            assert_eq!(
                scanner.tokens[0].kind,
                TokenKind::Identifier("identifier1".into())
            );
            assert_eq!(scanner.tokens[1].kind, TokenKind::Eof);
        }

        #[test]
        fn matches_identifiers_with_underscores() {
            let src = "identifier_one";
            let mut scanner = Scanner::new(src);

            scanner.scan_tokens();

            assert_eq!(
                scanner.tokens[0].kind,
                TokenKind::Identifier("identifier_one".into())
            );
        }

        #[test]
        fn doesnt_match_identifiers_with_leading_number() {
            let src = "1identifier";
            let mut scanner = Scanner::new(src);

            scanner.scan_tokens();

            assert!(matches!(scanner.tokens[0].kind, TokenKind::Number(_)));
            assert_eq!(
                scanner.tokens[1].kind,
                TokenKind::Identifier("identifier".into())
            );
        }
    }
}
