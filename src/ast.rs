pub trait TokenNode {
    fn token_literal(&self) -> String;
}

trait Statement {
    fn statement_node();
}

trait Expression {
    fn expression_node();
}

pub enum NodeTypes {
    ExpressionNode(ExpressionNode),
    StatementNode(StatementNode),
}

pub enum StatementNode {
    Root(RootStatement),
    Let(LetStatement),
}

impl TokenNode for StatementNode {
    fn token_literal(&self) -> String {
        return match self {
            StatementNode::Let(_) => "let".into(),
            StatementNode::Root(s) => s.token_literal(),
        };
    }
}

pub enum ExpressionNode {
    Nothing,
}

pub struct RootStatement {
    pub statements: Vec<StatementNode>,
}

impl TokenNode for RootStatement {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            return self.statements[0].token_literal();
        } else {
            return String::new();
        }
    }
}

pub struct LetStatement {
    pub name: IdentifierNode,
    pub value: ExpressionNode,
}

impl TokenNode for LetStatement {
    fn token_literal(&self) -> String {
        return self.name.token_literal();
    }
}

pub struct IdentifierNode {
    pub value: String,
}

impl TokenNode for IdentifierNode {
    fn token_literal(&self) -> String {
        return self.value.clone();
    }
}
