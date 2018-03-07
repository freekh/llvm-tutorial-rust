pub struct NumberExpr { val: f64 }
pub struct VariableExpr { name: String }
pub struct BinaryExpr { op: char, lhs: Box<AST>, rhs: Box<AST> }
pub struct CallExpr { callee: String, args: Vec<AST> }
pub struct Prototype { name: String, args: Vec<AST> }
pub struct Function { proto: Box<Prototype>, body: Box<AST> }

pub enum AST {
  NumberExpr(NumberExpr),
  VariableExpr(VariableExpr),
  BinaryExpr(BinaryExpr),
  CallExpr(CallExpr),
  Prototype(Prototype),
  Function(Function),
}
