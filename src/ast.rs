use color_eyre::eyre::Result;
use crate::lexer::Token;
use crate::utils::pop_front;

#[derive(Debug)]
pub enum ASTNode {
    Variable(String),
    Number(f64),

    BinaryExpr {
        lhs: Box<ASTNode>,
        op: Token,
        rhs: Box<ASTNode>,
    },
}

impl ASTNode {
    pub fn try_from(mut tokens: Vec<Token>) -> Result<Self> {
        Self::parse_program(&mut tokens)
    }

    fn parse_program(tokens: &mut Vec<Token>) -> Result<Self> {
        Self::parse_stmt(tokens)
    }

    fn parse_stmt(tokens: &mut Vec<Token>) -> Result<Self> {
        Self::parse_expr(tokens)
    }

    fn parse_expr(tokens: &mut Vec<Token>) -> Result<Self> {
        Self::parse_additive_expr(tokens)
    }

    fn parse_additive_expr(tokens: &mut Vec<Token>) -> Result<Self> {
        let mut lhs = Self::parse_multiplicative_expr(tokens)?;

        while tokens[0] == Token::OpAdd || tokens[0] == Token::OpSupstract {
            let op = pop_front(tokens).unwrap_or_else(|| unreachable!());
            let rhs = Self::parse_multiplicative_expr(tokens)?;

            lhs = Self::BinaryExpr {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_multiplicative_expr(tokens: &mut Vec<Token>) -> Result<Self> {
        let mut lhs = Self::parse_parenthesised_expr(tokens)?;

        while tokens[0] == Token::OpMultiply || tokens[0] == Token::OpDivide {
            let op = pop_front(tokens).unwrap_or_else(|| unreachable!());
            let rhs = Self::parse_parenthesised_expr(tokens)?;

            lhs = Self::BinaryExpr {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_parenthesised_expr(tokens: &mut Vec<Token>) -> Result<Self> {
        if tokens[0] != Token::OpenParen {
            let result = Self::parse_primary_expr(tokens)?;

            if tokens[0] != Token::OpenParen {
                return Ok(result);
            }

            // 1+1 (1-1)
            let rhs = Self::parse_expr(tokens)?;
            return Ok(Self::BinaryExpr {
                lhs: Box::new(result),
                op: Token::OpMultiply,
                rhs: Box::new(rhs),
            });
        }

        pop_front(tokens).unwrap_or_else(|| unreachable!());
        let result = Self::parse_expr(tokens)?;

        if tokens[0] != Token::CloseParen {
            return Err(color_eyre::Report::msg(format!(
                "Expected {:?} but found {:?}",
                Token::CloseParen,
                tokens[0]
            )));
        }
        pop_front(tokens);

        if tokens[0] == Token::OpenParen {
            // (1+1) (1-1)
            let rhs = Self::parse_expr(tokens)?;
            return Ok(Self::BinaryExpr {
                lhs: Box::new(result),
                op: Token::OpMultiply,
                rhs: Box::new(rhs),
            });
        }

        Ok(result)
    }

    fn parse_primary_expr(tokens: &mut Vec<Token>) -> Result<Self> {
        let token = match pop_front(tokens) {
            Some(token) => token,
            None => {
                return Err(color_eyre::Report::msg(format!(
                    "Unexpected end of vector {tokens:?}"
                )));
            },
        };

        match token {
            Token::Identifier(var_name) => Ok(Self::Variable(var_name)),
            Token::NumericLiteral(n) => Ok(Self::Number(n)),

            _ => todo!(),
        }
    }
}