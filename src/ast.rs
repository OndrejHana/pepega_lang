pub struct RootNode {
    pub statements: Vec<StatementNode>,
}

pub enum StatementNode {
    Let(LetStatement),
    Return(ReturnStatement),
}

pub struct ExpressionNode;

pub struct LetStatement {
    pub name: String,
    pub value: ExpressionNode,
}

pub struct ReturnStatement {
    pub return_value: ExpressionNode,
}
