//! Define all golang syntax node

use crate::token::Keyword;
use crate::token::LitKind;
use crate::token::Operator;

use std::fmt::Debug;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Comment {
    pub pos: usize,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Ident {
    pub pos: usize,
    pub name: String,
}

// ================ Type Definition ================

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointerType {
    pub pos: usize,
    pub typ: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayType {
    pub pos: (usize, usize),
    pub len: Box<Expression>,
    pub typ: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SliceType {
    pub pos: (usize, usize),
    pub typ: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapType {
    pub pos: (usize, usize),
    pub key: Box<Expression>,
    pub val: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Field {
    pub name: Vec<Ident>,
    pub typ: Expression,
    pub tag: Option<StringLit>,
    pub comments: Vec<Rc<Comment>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FieldList {
    pub pos: Option<(usize, usize)>,
    pub list: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructType {
    pub pos: (usize, usize),
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FuncType {
    pub pos: usize,
    pub typ_params: FieldList,
    pub params: FieldList,
    pub result: FieldList,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChanMode {
    Send,
    Recv,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChannelType {
    pub pos: (usize, usize), // chan, <-
    pub dir: Option<ChanMode>,
    pub typ: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InterfaceType {
    pub pos: usize,
    pub methods: FieldList,
}

// ================ Expression Definition ================

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BasicLit {
    pub pos: usize,
    pub kind: LitKind,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StringLit {
    pub pos: usize,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncLit {
    pub typ: FuncType,
    pub body: BlockStmt,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Element {
    Expr(Expression),
    LitValue(LiteralValue),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyedElement {
    pub key: Option<Element>,
    pub val: Element,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiteralValue {
    pub pos: (usize, usize),
    pub values: Vec<KeyedElement>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompositeLit {
    pub typ: Box<Expression>,
    pub val: LiteralValue,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Selector {
    pub pos: usize,
    pub x: Box<Expression>,
    pub sel: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeAssertion {
    pub pos: (usize, usize),
    pub left: Box<Expression>,
    pub right: Option<Box<Expression>>, // None for x.(type)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index {
    pub pos: (usize, usize),
    pub left: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexList {
    pub pos: (usize, usize),
    pub left: Box<Expression>,
    pub indices: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slice {
    pub pos: (usize, usize),
    pub left: Box<Expression>,
    pub index: [Option<Box<Expression>>; 3],
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Call {
    pub pos: (usize, usize), // third pos > 0 means the ellipsis argument
    pub args: Vec<Expression>,
    pub func: Box<Expression>,
    pub dots: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParenExpression {
    pub pos: (usize, usize),
    pub expr: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StarExpression {
    pub pos: usize,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ellipsis {
    pub pos: usize,
    pub elt: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RangeExpr {
    pub pos: usize, // pos of 'range'
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Operation {
    pub pos: usize,
    pub op: Operator,
    pub x: Box<Expression>,
    pub y: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expression {
    Call(Call),
    Index(Index),
    IndexList(IndexList),
    Slice(Slice),
    Ident(Ident),
    FuncLit(FuncLit),
    Ellipsis(Ellipsis),
    Selector(Selector),
    BasicLit(BasicLit),
    Range(RangeExpr),
    Star(StarExpression),
    Paren(ParenExpression),
    TypeAssert(TypeAssertion),
    CompositeLit(CompositeLit),
    List(Vec<Expression>),
    Operation(Operation),
    TypeMap(MapType),             // map[K]V
    TypeArray(ArrayType),         // [N]T
    TypeSlice(SliceType),         // []T
    TypeFunction(FuncType),       // func (...) ...
    TypeStruct(StructType),       // struct { ... }
    TypeChannel(ChannelType),     // <-chan T | chan<- T | chan T
    TypePointer(PointerType),     // *T
    TypeInterface(InterfaceType), // interface { ... }
}

// ================ Declaration Definition ================

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decl<T> {
    pub docs: Vec<Rc<Comment>>,
    pub pos0: usize,                  // pos of var | const | type
    pub pos1: Option<(usize, usize)>, // pos of '(' and ')'
    pub specs: Vec<T>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VarSpec {
    pub docs: Vec<Rc<Comment>>,
    pub name: Vec<Ident>,
    pub typ: Option<Expression>,
    pub values: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ConstSpec {
    pub docs: Vec<Rc<Comment>>,
    pub name: Vec<Ident>,
    pub typ: Option<Expression>,
    pub values: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeSpec {
    pub docs: Vec<Rc<Comment>>,
    pub alias: bool,
    pub name: Ident,
    pub params: FieldList,
    pub typ: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FuncDecl {
    pub docs: Vec<Rc<Comment>>,
    pub recv: Option<FieldList>,
    pub name: Ident,
    pub typ: FuncType,
    pub body: Option<BlockStmt>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Declaration {
    Function(FuncDecl),
    Type(Decl<TypeSpec>),
    Const(Decl<ConstSpec>),
    Variable(Decl<VarSpec>),
}

// ================ Statement Definition ================

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockStmt {
    pub pos: (usize, usize),
    pub list: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeclStmt {
    Type(Decl<TypeSpec>),
    Const(Decl<ConstSpec>),
    Variable(Decl<VarSpec>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GoStmt {
    pub pos: usize,
    pub call: Call,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeferStmt {
    pub pos: usize,
    pub call: Call,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReturnStmt {
    pub pos: usize,
    pub ret: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BranchStmt {
    pub pos: usize,
    pub key: Keyword,
    pub ident: Option<Ident>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IfStmt {
    pub pos: usize,
    pub init: Option<Box<Statement>>,
    pub cond: Expression,
    pub body: BlockStmt,
    pub else_: Option<Box<Statement>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssignStmt {
    // position of assign operator like = | += | &=
    pub pos: usize,
    pub op: Operator,
    pub left: Vec<Expression>,
    pub right: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LabeledStmt {
    pub pos: usize,
    pub name: Ident,
    pub stmt: Box<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendStmt {
    pub pos: usize,
    pub chan: Expression,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprStmt {
    pub expr: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CaseClause {
    pub tok: Keyword,
    pub pos: (usize, usize),
    pub list: Vec<Expression>,
    pub body: Box<Vec<Statement>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CaseBlock {
    pub pos: (usize, usize),
    pub body: Vec<CaseClause>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwitchStmt {
    pub pos: usize,
    pub init: Option<Box<Statement>>,
    pub tag: Option<Expression>,
    pub block: CaseBlock,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeSwitchStmt {
    pub pos: usize,
    pub init: Option<Box<Statement>>,
    pub tag: Option<Box<Statement>>,
    pub block: CaseBlock,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IncDecStmt {
    pub pos: usize,
    pub op: Operator,
    pub expr: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommClause {
    pub pos: (usize, usize), // pos of (keyword, colon)
    pub tok: Keyword,
    pub comm: Option<Box<Statement>>,
    pub body: Box<Vec<Statement>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommBlock {
    pub pos: (usize, usize),
    pub body: Vec<CommClause>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelectStmt {
    pub pos: usize,
    pub body: CommBlock,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RangeStmt {
    pub pos: (usize, usize), // pos of (for, range)
    pub key: Option<Expression>,
    pub value: Option<Expression>,
    pub op: Option<(usize, Operator)>, // define or assign
    pub expr: Expression,
    pub body: BlockStmt,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ForStmt {
    pub pos: usize,
    pub init: Option<Box<Statement>>,
    pub cond: Option<Box<Statement>>,
    pub post: Option<Box<Statement>>,
    pub body: BlockStmt,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmptyStmt {
    pub pos: usize,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Statement {
    Go(GoStmt),
    If(IfStmt),
    For(ForStmt),
    Send(SendStmt),
    Expr(ExprStmt),
    Defer(DeferStmt),
    Block(BlockStmt),
    Range(RangeStmt),
    Empty(EmptyStmt),
    Label(LabeledStmt),
    IncDec(IncDecStmt),
    Assign(AssignStmt),
    Return(ReturnStmt),
    Branch(BranchStmt),
    Switch(SwitchStmt),
    Select(SelectStmt),
    TypeSwitch(TypeSwitchStmt),
    Declaration(DeclStmt),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Import {
    pub name: Option<Ident>,
    pub path: StringLit,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct File {
    pub path: PathBuf,
    pub line_info: Vec<usize>,
    pub docs: Vec<Rc<Comment>>,
    pub pkg_name: Ident,
    pub imports: Vec<Import>,
    pub decl: Vec<Declaration>,
    pub comments: Vec<Rc<Comment>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Package {
    pub path: PathBuf,
    pub files: Vec<File>,
}

// ================ Type Implemention ================

impl From<Ident> for Field {
    fn from(id: Ident) -> Self {
        Self {
            name: vec![],
            typ: Expression::Ident(id),
            tag: None,
            comments: Default::default(),
        }
    }
}

impl From<BasicLit> for StringLit {
    fn from(lit: BasicLit) -> StringLit {
        assert_eq!(lit.kind, LitKind::String);
        StringLit { pos: lit.pos, value: lit.value }
    }
}

impl From<Expression> for Field {
    fn from(expr: Expression) -> Self {
        Self {
            name: vec![],
            typ: expr,
            tag: None,
            comments: Default::default(),
        }
    }
}

impl AssignStmt {
    pub fn is_range(&self) -> bool {
        matches!(self.right.first(), Some(Expression::Range(_)))
    }
}

impl FieldList {
    pub fn pos(&self) -> usize {
        if let Some((pos, _)) = self.pos {
            return pos;
        }

        if let Some(field) = self.list.first() {
            if let Some(name) = field.name.first() {
                return name.pos;
            }

            return field.typ.pos();
        }

        panic!("call pos on empty FieldList");
    }
}

impl Expression {
    pub fn pos(&self) -> usize {
        match self {
            Expression::Call(x) => x.pos.0,
            Expression::Index(x) => x.pos.0,
            Expression::Slice(x) => x.pos.0,
            Expression::Ident(x) => x.pos,
            Expression::FuncLit(x) => x.typ.pos,
            Expression::Ellipsis(x) => x.pos,
            Expression::Selector(x) => x.x.pos(),
            Expression::BasicLit(x) => x.pos,
            Expression::Range(x) => x.pos,
            Expression::Star(x) => x.pos,
            Expression::Paren(x) => x.pos.0,
            Expression::TypeAssert(x) => x.left.pos(),
            Expression::CompositeLit(x) => x.typ.pos(),
            Expression::IndexList(x) => x.left.pos(),
            Expression::List(_) => unimplemented!("list may empty"),
            Expression::Operation(x) => x.x.pos(),
            Expression::TypeMap(x) => x.pos.0,
            Expression::TypeArray(x) => x.pos.0,
            Expression::TypeSlice(x) => x.pos.0,
            Expression::TypeFunction(f) => f.pos,
            Expression::TypeStruct(x) => x.pos.0,
            Expression::TypeChannel(x) => x.pos.0,
            Expression::TypePointer(x) => x.pos,
            Expression::TypeInterface(x) => x.pos,
        }
    }
}

pub(crate) trait Spec {
    fn with_docs(self, docs: Vec<Rc<Comment>>) -> Self;
}

impl Spec for VarSpec {
    fn with_docs(mut self, docs: Vec<Rc<Comment>>) -> VarSpec {
        self.docs = docs;
        self
    }
}

impl Spec for TypeSpec {
    fn with_docs(mut self, docs: Vec<Rc<Comment>>) -> TypeSpec {
        self.docs = docs;
        self
    }
}

impl Spec for ConstSpec {
    fn with_docs(mut self, docs: Vec<Rc<Comment>>) -> ConstSpec {
        self.docs = docs;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_source;

    #[test]
    fn show_debug() {
        let source = include_str!("../tests/testdata/helloworld.go");
        let s = parse_source(source).unwrap();
        println!("{:#?}", s);
    }
}
