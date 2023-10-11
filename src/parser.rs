use std::mem::discriminant;

use crate::ast::*;
use crate::lexer::*;

use anyhow::bail;
use anyhow::Result;

pub struct Parser {
    l: Lexer,
    curr: Token,
    peek: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(l: Lexer) -> Self {
        let mut p = Self {
            l,
            curr: Token::Illegal,
            peek: Token::Illegal,
            errors: Vec::new(),
        };

        p.next_token();
        p.next_token();

        return p;
    }

    fn peek_error(&mut self, expected: Token) {
        let msg = format!(
            "expected next token to be: {:?}, got {:?} instead",
            expected, self.peek
        );
        self.errors.push(msg);
    }

    fn next_token(&mut self) {
        self.curr = self.peek.clone();
        self.peek = self.l.next_token();
    }

    fn expect_next(&mut self, t: Token) -> bool {
        if discriminant(&self.peek) == discriminant(&t) {
            self.next_token();
            return true;
        }

        self.peek_error(t);
        return false;
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement> {
        let ident = if self.expect_next(Token::Identifier(String::new())) {
            if let Token::Identifier(ident) = &self.curr {
                ident.clone()
            } else {
                bail!("identifier for let statement not found");
            }
        } else {
            bail!("identifier for let statement not found");
        };

        if !self.expect_next(Token::Assign) {
            bail!("assign for let statement not found");
        }

        while self.curr != Token::Semicolon {
            self.next_token();
        }

        return Ok(LetStatement {
            name: ident,
            value: ExpressionNode,
        });
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement> {
        self.next_token();

        let return_statement = ReturnStatement {
            return_value: ExpressionNode,
        };

        while self.curr != Token::Semicolon {
            self.next_token();
        }

        return Ok(return_statement);
    }

    fn parse_statement(&mut self) -> Result<StatementNode> {
        return match self.curr {
            Token::Let => Ok(StatementNode::Let(self.parse_let_statement()?)),
            Token::Return => Ok(StatementNode::Return(self.parse_return_statement()?)),
            _ => bail!("node for token not found"),
        };
    }

    pub fn parse_root(&mut self) -> Result<RootNode> {
        let mut root = RootNode {
            statements: Vec::new(),
        };

        while self.curr != Token::EoF {
            if let Ok(st_node) = self.parse_statement() {
                root.statements.push(st_node);
            }
            self.next_token();
        }

        return Ok(root);
    }
}

#[cfg(test)]
mod test {
    use anyhow::{bail, Ok, Result};

    use crate::ast::StatementNode;
    use crate::lexer::Lexer;

    use super::Parser;

    #[test]
    fn test_parser1() -> Result<()> {
        let input = "\
let x = 5;
let y = 10;
let foobar = 838383;";

        let l = Lexer::new(input.into());
        let mut p = Parser::new(l);

        let root = p.parse_root()?;
        check_errors(&p)?;

        if root.statements.len() != 3 {
            bail!(
                "Root statement does not contain 3 statement nodes {}",
                root.statements.len()
            );
        }

        let tests = vec!["x", "y", "foobar"];

        for (i, test) in tests.iter().enumerate() {
            let statement = &root.statements[i];
            test_let_statement(statement, test)?;
        }

        return Ok(());
    }

    #[test]
    fn test_parser2() -> Result<()> {
        let input = "\
let x 5;
let = 10;
let 838383;";

        let l = Lexer::new(input.into());
        let mut p = Parser::new(l);

        let root = p.parse_root()?;

        assert_eq!(p.errors.len(), 3);

        return Ok(());
    }

    fn test_let_statement(s: &StatementNode, test: &str) -> Result<()> {
        if let StatementNode::Let(let_statement) = s {
            if let_statement.name != test {
                bail!("name does not match");
            }
        } else {
            bail!("invalid token literal");
        }

        return Ok(());
    }

    fn check_errors(p: &Parser) -> Result<()> {
        if p.errors.is_empty() {
            return Ok(());
        }

        bail!(
            "encountered {} errors: {}",
            p.errors.len(),
            p.errors.join("\n")
        );
    }

    #[test]
    fn test_return_statement() -> Result<()> {
        let input = "\
return 5;
return 10;
return 993322;";

        let l = Lexer::new(input.into());
        let mut p = Parser::new(l);

        let root = p.parse_root()?;

        check_errors(&p)?;

        if root.statements.len() != 3 {
            bail!(
                "Root does not contain 3 nodes, {} instead ",
                root.statements.len()
            );
        }

        for node in root.statements {
            if let StatementNode::Return(_) = node {
                continue;
            }

            bail!("Incorrect node!");
        }

        return Ok(());
    }
}
