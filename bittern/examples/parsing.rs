// Demonstrates a simple expression interpreter using an arena-allocated syntax tree.
// The language uses Lisp-like prefix notation with optional parentheses

use bittern::{Arena, Item, Rel, SecondaryMap};
use core::hash::Hash;
use core::ops::{Add, Sub, Mul, Div};

fn main() {
    // Evaluate the Pythagorean theorem (sqrt(3000^2 + 6000^2) = 6708)
    let input = r#"
    do
    let a 3000
    let b 6000
    let c sqrt (+ (pow a 2) (pow b 2))
    (c)
    "#;
    let mut parser = Parser::new();
    let expr = parser.parse(input);
    assert_eq!(parser.expr_table.len(), 14);

    let result = Eval::new().eval(expr);
    assert_eq!(result, Some(6708));
}

type Int = i64;
type Name = str;

// A single token produced by the lexer
#[derive(Hash, PartialEq, Eq, Debug)]
enum Token {
    ParenOpen,
    ParenClose,
    Int(Int),
    Name(Item<Name>),
    Do,
    Let,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Sqrt,
}

// An expression tree or subtree.
// Item<Name> is a strong ref, so the Name arena will live until the Expr is dropped.
// Rel<Expr> is a weak ref, so expressions may reference others within the same arena.
#[derive(Hash, PartialEq, Eq, Debug)]
enum Expr {
    Empty,
    Int(Int),
    Name(Item<Name>),
    Block(Vec<Rel<Expr>>),
    Let(Rel<Expr>, Rel<Expr>),
    Add(Rel<Expr>, Rel<Expr>),
    Sub(Rel<Expr>, Rel<Expr>),
    Mul(Rel<Expr>, Rel<Expr>),
    Div(Rel<Expr>, Rel<Expr>),
    Pow(Rel<Expr>, Rel<Expr>),
    Sqrt(Rel<Expr>),
}

// Evaluates the AST.
// SecondaryMap associates an Item<Name> with a value
struct Eval {
    var_table: SecondaryMap<Name, Option<Int>>,
}
impl Eval {
    fn new() -> Self {
        Self {
            var_table: SecondaryMap::new(),
        }
    }

    fn eval(&mut self, expr: Item<Expr>) -> Option<Int> {
        fn pow(lhs: Int, rhs: Int) -> Int {
            lhs.pow(rhs.clamp(u32::MIN as i64, u32::MAX as i64) as u32)
        }

        match &*expr {
            Expr::Empty => None,
            Expr::Int(i) => Some(*i),
            Expr::Name(name) => self.var(name),
            Expr::Block(exprs) => self.eval_block(exprs),
            Expr::Let(lhs, rhs) => self.eval_let(lhs, rhs),
            Expr::Add(lhs, rhs) => self.eval_binary(Int::add, lhs, rhs),
            Expr::Sub(lhs, rhs) => self.eval_binary(Int::sub, lhs, rhs),
            Expr::Mul(lhs, rhs) => self.eval_binary(Int::mul, lhs, rhs),
            Expr::Div(lhs, rhs) => self.eval_binary(Int::div, lhs, rhs),
            Expr::Pow(lhs, rhs) => self.eval_binary(pow, lhs, rhs),
            Expr::Sqrt(rel) => self.eval_unary(Int::isqrt, rel),
        }
    }

    fn var(&self, name: &Item<Name>) -> Option<Int> {
        match self.var_table.get(name) {
            None => None,
            Some(val) => *val,
        }
    }

    fn eval_block(&mut self, exprs: &[Rel<Expr>]) -> Option<Int> {
        if exprs.is_empty() {
            return None;
        }
        for i in 0..(exprs.len() - 1) {
            let expr = (&exprs[i]).item().expect("invalid relation");
            self.eval(expr);
        }
        let expr = (&exprs[exprs.len() - 1]).item().expect("invalid relation");
        self.eval(expr)
    }

    fn eval_let(&mut self, lhs_rel: &Rel<Expr>, rhs_rel: &Rel<Expr>) -> Option<Int> {
        let lhs = lhs_rel.item().expect("invalid relation");
        let Expr::Name(name) = &*lhs else {
            panic!("lhs of let statement must be a name");
        };
        if self.var_table.contains(name) {
            panic!("name `{}` is assigned more than once", name);
        }
        let rhs = rhs_rel.item().expect("invalid relation");
        let rhs_val = self.eval(rhs);
        self.var_table.insert(name.clone(), rhs_val);
        rhs_val
    }

    fn eval_unary(&mut self, op: fn(Int) -> Int, rel: &Rel<Expr>) -> Option<Int> {
        let expr = rel.item().expect("invalid relation");
        let Some(val) = self.eval(expr) else {
            return None;
        };
        Some(op(val))
    }

    fn eval_binary(&mut self, op: fn(Int, Int) -> Int, lhs_rel: &Rel<Expr>, rhs_rel: &Rel<Expr>) -> Option<Int> {
        let lhs = lhs_rel.item().expect("invalid relation");
        let rhs = rhs_rel.item().expect("invalid relation");
        let Some(lhs_val) = self.eval(lhs) else {
            return None;
        };
        let Some(rhs_val) = self.eval(rhs) else {
            return None;
        };
        Some(op(lhs_val, rhs_val))
    }
}


// Parses input into an AST.
// Identical expressions will be interned into a single node.
struct Parser<'src> {
    input: &'src str,
    name_table: Arena<Name>,
    expr_table: Arena<Expr>,
}
impl<'src> Parser<'src> {
    fn new() -> Self {
        Self {
            input: "",
            name_table: Arena::new(),
            expr_table: Arena::new(),
        }
    }

    // Parsing methods:

    fn parse(&mut self, input: &'src str) -> Item<Expr> {
        self.input = input;
        let expr = self.expr();
        if !self.input.is_empty() {
            panic!("expected EOF");
        }
        expr
    }

    fn expr(&mut self) -> Item<Expr> {
        let Some(token) = self.next_token() else {
            panic!("unexpected EOF");
        };
        self.expr_start(token)
    }

    fn expr_start(&mut self, token: Token) -> Item<Expr> {
        match token {
            Token::ParenClose => panic!("unexpected closing paren"),
            Token::ParenOpen => match self.next_token() {
                None => panic!("unexpected EOF"),
                Some(Token::ParenClose) => self.empty(),
                Some(token) => {
                    let expr = self.expr_start(token);
                    let Some(Token::ParenClose) = self.next_token() else {
                        panic!("expected closing paren");
                    };
                    expr
                },
            },
            Token::Int(i) => self.int(i),
            Token::Name(s) => self.name(s),
            Token::Do => self.op_many(Expr::Block),
            Token::Let => self.op_binary(Expr::Let),
            Token::Add => self.op_binary(Expr::Add),
            Token::Sub => self.op_binary(Expr::Sub),
            Token::Mul => self.op_binary(Expr::Mul),
            Token::Div => self.op_binary(Expr::Div),
            Token::Pow => self.op_binary(Expr::Pow),
            Token::Sqrt => self.op_unary(Expr::Sqrt),
        }
    }

    fn empty(&self) -> Item<Expr> {
        self.expr_table.intern_owned(Expr::Empty)
    }

    fn int(&self, i: Int) -> Item<Expr> {
        self.expr_table.intern_owned(Expr::Int(i))
    }

    fn name(&self, s: Item<Name>) -> Item<Expr> {
        self.expr_table.intern_owned(Expr::Name(s))
    }

    fn op_unary(&mut self, op: fn(Rel<Expr>) -> Expr) -> Item<Expr> {
        let val = self.expr();
        self.expr_table.intern_owned(op(val.rel()))
    }

    fn op_binary(&mut self, op: fn(Rel<Expr>, Rel<Expr>) -> Expr) -> Item<Expr> {
        let lhs = self.expr();
        let rhs = self.expr();
        self.expr_table.intern_owned(op(lhs.rel(), rhs.rel()))
    }

    fn op_many(&mut self, op: fn(Vec<Rel<Expr>>) -> Expr) -> Item<Expr> {
        let mut exprs: Vec<Rel<Expr>> = Vec::new();
        loop {
            if matches!(self.peek_char(), None | Some(')')) {
                break;
            }
            let expr = self.expr();
            exprs.push(expr.rel());
        }
        self.expr_table.intern_owned(op(exprs))
    }

    // Lexing methods:

    fn read_until<P>(&mut self, pat: P) -> &'src str
    where P: Fn(char) -> bool
    {
        match self.input.find(pat) {
            None => {
                let s = self.input;
                self.input = "";
                s
            }
            Some(end) => {
                let s = &self.input[..end];
                self.input = &self.input[end..];
                s
            }
        }
    }
    fn read_while<P>(&mut self, pat: P) -> &'src str
    where P: Fn(char) -> bool
    {
        self.read_until(|c| !pat(c))
    }

    fn peek_char(&mut self) -> Option<char> {
        self.input.chars().next()
    }

    fn ignore_whitespace(&mut self) {
        self.read_while(char::is_whitespace);
    }

    fn next_token(&mut self) -> Option<Token> {
        self.ignore_whitespace();
        let token = match self.peek_char()? {
            '(' => {
                self.input = &self.input[1..];
                Token::ParenOpen
            },
            ')' => {
                self.input = &self.input[1..];
                Token::ParenClose
            },
            c => {
                let word = self.read_until(|c| c == '(' || c == ')' || c.is_whitespace());
                if c.is_digit(10) {
                    let i = word.parse::<Int>().expect("invalid int literal");
                    Token::Int(i)
                } else {
                    match word {
                        "do" => Token::Do,
                        "let" => Token::Let,
                        "add" | "+" => Token::Add,
                        "sub" | "-" => Token::Sub,
                        "mul" | "*" => Token::Mul,
                        "div" | "/" => Token::Div,
                        "pow" | "**" => Token::Pow,
                        "sqrt" => Token::Sqrt,
                        _ => Token::Name(self.name_table.intern(word))
                    }
                }
            }
        };
        self.ignore_whitespace();
        Some(token)
    }
}