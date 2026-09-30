// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 2.58, one module and one test.

mod ex_2_58 {
    use ch02::sec_2_3::{Expr, deriv, make_product, make_sum};
    use sicp_runtime::{SicpError, Symbol};

    /// One infix token: this exercise's own contribution is turning
    /// concrete infix syntax into the section's existing `Expr`, so
    /// `deriv` and the two constructors need no change at all.
    #[derive(Clone, Debug, PartialEq)]
    enum Token {
        Num(i128),
        Var(Symbol),
        Plus,
        Star,
        LParen,
        RParen,
    }

    fn tokenize(input: &str) -> Result<Vec<Token>, SicpError> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
                continue;
            }
            let single = match c {
                '(' => Some(Token::LParen),
                ')' => Some(Token::RParen),
                '+' => Some(Token::Plus),
                '*' => Some(Token::Star),
                _ => None,
            };
            if let Some(token) = single {
                chars.next();
                tokens.push(token);
                continue;
            }
            let mut word = String::new();
            while let Some(&c2) = chars.peek() {
                if c2.is_whitespace() || "()+*".contains(c2) {
                    break;
                }
                word.push(c2);
                chars.next();
            }
            if word.is_empty() {
                return Err(SicpError::Parse(format!("unexpected character: {c}")));
            }
            tokens.push(
                word.parse::<i128>()
                    .map_or_else(|_| Token::Var(Symbol::from(word.as_str())), Token::Num),
            );
        }
        Ok(tokens)
    }

    struct Parser<'a> {
        tokens: &'a [Token],
        pos: usize,
    }

    impl<'a> Parser<'a> {
        fn new(tokens: &'a [Token]) -> Self {
            Parser { tokens, pos: 0 }
        }

        fn peek(&self) -> Result<&Token, SicpError> {
            self.tokens
                .get(self.pos)
                .ok_or_else(|| SicpError::Parse("unexpected end of input".to_string()))
        }

        fn advance(&mut self) -> Result<Token, SicpError> {
            let token = self.peek()?.clone();
            self.pos += 1;
            Ok(token)
        }

        fn expect(&mut self, expected: &Token) -> Result<(), SicpError> {
            let found = self.advance()?;
            if &found == expected {
                Ok(())
            } else {
                Err(SicpError::Parse(format!(
                    "expected {expected:?}, found {found:?}"
                )))
            }
        }

        /// An atom: a number or a variable.
        fn atom(&mut self) -> Result<Expr, SicpError> {
            match self.advance()? {
                Token::Num(n) => Ok(Expr::Num(n)),
                Token::Var(s) => Ok(Expr::Var(s)),
                other => Err(SicpError::Parse(format!(
                    "expected an atom, found {other:?}"
                ))),
            }
        }

        /// Part a: fully parenthesized infix. Every binary operation
        /// is wrapped in its own parentheses, so no precedence rule is
        /// needed: `"(" expr op expr ")"`, or a bare atom.
        fn fully_parenthesized(&mut self) -> Result<Expr, SicpError> {
            if self.peek()? != &Token::LParen {
                return self.atom();
            }
            self.expect(&Token::LParen)?;
            let left = self.fully_parenthesized()?;
            let op = self.advance()?;
            let right = self.fully_parenthesized()?;
            self.expect(&Token::RParen)?;
            match op {
                Token::Plus => make_sum(left, right),
                Token::Star => make_product(left, right),
                other => Err(SicpError::Parse(format!(
                    "expected + or *, found {other:?}"
                ))),
            }
        }

        /// Part b: ordinary infix, `*` binding tighter than `+`, and
        /// parentheses only where needed to override precedence:
        /// `expr := term ("+" term)*`, `term := factor ("*" factor)*`,
        /// `factor := atom | "(" expr ")"`.
        fn precedence_expr(&mut self) -> Result<Expr, SicpError> {
            let mut left = self.precedence_term()?;
            while self.pos < self.tokens.len() && self.peek()? == &Token::Plus {
                self.advance()?;
                let right = self.precedence_term()?;
                left = make_sum(left, right)?;
            }
            Ok(left)
        }

        fn precedence_term(&mut self) -> Result<Expr, SicpError> {
            let mut left = self.precedence_factor()?;
            while self.pos < self.tokens.len() && self.peek()? == &Token::Star {
                self.advance()?;
                let right = self.precedence_factor()?;
                left = make_product(left, right)?;
            }
            Ok(left)
        }

        fn precedence_factor(&mut self) -> Result<Expr, SicpError> {
            if self.peek()? == &Token::LParen {
                self.advance()?;
                let inner = self.precedence_expr()?;
                self.expect(&Token::RParen)?;
                return Ok(inner);
            }
            self.atom()
        }
    }

    fn parse_fully_parenthesized(input: &str) -> Result<Expr, SicpError> {
        let tokens = tokenize(input)?;
        Parser::new(&tokens).fully_parenthesized()
    }

    fn parse_infix(input: &str) -> Result<Expr, SicpError> {
        let tokens = tokenize(input)?;
        Parser::new(&tokens).precedence_expr()
    }

    /// Exercise 2.58: infix notation
    ///
    /// Returns the printed derivative of the fully parenthesized
    /// expression `(x + (3 * (x + (y + 2))))` with respect to `x`
    /// (part a), and the printed derivative of the ordinary-notation
    /// expression `x + 3 * (x + y + 2)` with respect to `x` (part b),
    /// in that order; both fold to the same answer.
    pub fn ex_2_58() -> (String, String) {
        let x = Symbol::from("x");
        let a = parse_fully_parenthesized("(x + (3 * (x + (y + 2))))").expect("well-formed input");
        let b = parse_infix("x + 3 * (x + y + 2)").expect("well-formed input");
        (
            deriv(&a, &x).expect("no overflow").to_string(),
            deriv(&b, &x).expect("no overflow").to_string(),
        )
    }
}

#[test]
fn ex_2_58() {
    assert_eq!(ex_2_58::ex_2_58(), ("4".to_string(), "4".to_string()));
}
