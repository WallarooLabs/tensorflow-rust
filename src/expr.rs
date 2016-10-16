// -*-  indent-tabs-mode:nil; tab-width:2;  -*-
//! This module builds computation graphs.
//!
//! This module is unfinished.
#![cfg(feature = "tensorflow_unstable")]

use std::cmp::Eq;
use std::collections::HashMap;
use std::convert::From;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::ops;
use std::rc::Rc;
use super::DataType;
use super::Graph;
use super::Operation;
use super::Port;
use super::Status;
use super::Tensor;
use super::TensorType;

/// Denotes operator precedence.
/// Used for displaying expressions as strings.
#[derive(Ord,PartialOrd,Eq,PartialEq,Debug)]
pub enum OpLevel {
  /// Assignment.
  Assign,

  /// Addition and subtraction.
  Add,

  /// Multiplication, division, and remainder.
  Mul,

  /// Unary operators like negation.
  Unary,

  /// Variables and constants.
  Atom,
}

////////////////////////

/// A operation in an expression tree, which is a thin wrapper around an ExprImpl.
///
/// This is separate from ExprImpl because we want expressions to be wrapped in an Rc,
/// and we can't directly implement std::ops::Add, etc., for Rc<E: ExprImpl<T>>.
#[derive(Debug,Clone)]
pub struct Expr<T: TensorType> {
  expr: Rc<ExprImpl<T>>,
}

impl<T: TensorType> Display for Expr<T> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    Display::fmt(&self.expr, f)
  }
}

impl<T: TensorType> From<T> for Expr<T> {
  fn from(value: T) -> Self {
    Expr {
      expr: Rc::new(value),
    }
  }
}

////////////////////////

/// Trait implemented by all expression types.
/// Most users will want to store an Expr instead.
pub trait ExprImpl<T: TensorType>: Display + Debug {
  /// Returns the precedence level for this operator.
  fn op_level(&self) -> OpLevel;

  /// Returns the child expressions.
  ///
  /// For example, the child expressions of `x + y` would be `x` and `y`.
  fn children(&self) -> Vec<Box<AnyExpr>>; // TODO: return an iterator

  /// Creates an operation for the expression.
  ///
  /// The implementation must use the operations in the `children` parameter
  /// rather than creating child operations itself.
  fn create_operation(&self, graph: &mut Graph, children: &[Rc<Operation>], id_gen: &mut FnMut() -> String) -> Result<Operation, Status>;
}

impl<T: TensorType> ExprImpl<T> for T {
  fn op_level(&self) -> OpLevel {
    OpLevel::Atom
  }

  fn children(&self) -> Vec<Box<AnyExpr>> {
    vec![]
  }

  fn create_operation(&self, graph: &mut Graph, _children: &[Rc<Operation>], id_gen: &mut FnMut() -> String) -> Result<Operation, Status> {
    let mut nd = try!(graph.new_operation("Const", &id_gen()));
    try!(nd.set_attr_type("dtype", DataType::Float));
    let mut value = Tensor::new(&[1]);
    value[0] = *self;
    try!(nd.set_attr_tensor("value", value));
    nd.finish()
  }
}

////////////////////////

macro_rules! impl_bin_op {
  ($name:ident, $fn_name:ident, $op:expr, $op_level:ident, $assoc:expr, $tf_op:expr, $doc:expr) => {
    #[doc = $doc]
    #[derive(Debug)]
    pub struct $name<T: TensorType> {
      left: Expr<T>,
      right: Expr<T>,
    }

    impl<T: TensorType> ops::$name for Expr<T> {
      type Output = Expr<T>;

      fn $fn_name(self, rhs: Expr<T>) -> Expr<T> {
        Expr {
          expr: Rc::new($name {
            left: self,
            right: rhs,
          }),
        }
      }
    }

    impl<T: TensorType> ops::$name<T> for Expr<T> {
      type Output = Expr<T>;

      fn $fn_name(self, rhs: T) -> Expr<T> {
        Expr {
          expr: Rc::new($name {
            left: self,
            right: Expr::from(rhs),
          }),
        }
      }
    }

    impl<T: TensorType> Display for $name<T> {
      fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if self.left.expr.op_level() < OpLevel::$op_level {
          try!(write!(f, "({})", self.left));
        } else {
          try!(write!(f, "{}", self.left));
        }
        try!(write!(f, concat!(" ", $op, " ")));
        let paren = if $assoc {
          self.right.expr.op_level() < OpLevel::$op_level
        } else {
          self.right.expr.op_level() <= OpLevel::$op_level
        };
        if paren {
          write!(f, "({})", self.right)
        } else {
          write!(f, "{}", self.right)
        }
      }
    }

    impl<T: TensorType> ExprImpl<T> for $name<T> {
      fn op_level(&self) -> OpLevel {
        OpLevel::$op_level
      }

      fn children(&self) -> Vec<Box<AnyExpr>> {
        vec![Box::new(self.left.clone()), Box::new(self.right.clone())]
      }

      fn create_operation(&self, graph: &mut Graph, children: &[Rc<Operation>], id_gen: &mut FnMut() -> String) -> Result<Operation, Status> {
        let mut nd = try!(graph.new_operation($tf_op, &id_gen()));
        nd.add_input(Port {operation: &children[0], index: 0});
        nd.add_input(Port {operation: &children[1], index: 0});
        nd.finish()
      }
    }
  }
}

impl_bin_op!(Add, add, "+", Add, true, "Add", "Expression resulting from adding two subexpressions.");
impl_bin_op!(Sub, sub, "-", Add, false, "Sub", "Expression resulting from subtracting two subexpressions.");
impl_bin_op!(Mul, mul, "*", Mul, true, "Mul", "Expression resulting from multiplying two subexpressions.");
impl_bin_op!(Div, div, "/", Mul, false, "Div", "Expression resulting from dividing two subexpressions.");
impl_bin_op!(Rem, rem, "%", Mul, false, "Mod", "Expression resulting from taking a modulus.");

////////////////////////

/// Expression resulting from negation of an expression.
#[derive(Debug)]
pub struct Neg<T: TensorType> {
  expr: Expr<T>,
}

impl<T: TensorType> ops::Neg for Expr<T> {
  type Output = Expr<T>;

  fn neg(self) -> Expr<T> {
    Expr {
      expr: Rc::new(Neg {
        expr: self,
      }),
    }
  }
}

impl<T: TensorType> Display for Neg<T> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    try!(write!(f, "-"));
    if self.expr.expr.op_level() <= OpLevel::Unary {
      write!(f, "({})", self.expr)
    } else {
      write!(f, "{}", self.expr)
    }
  }
}

impl<T: TensorType> ExprImpl<T> for Neg<T> {
  fn op_level(&self) -> OpLevel {
    OpLevel::Unary
  }

  fn children(&self) -> Vec<Box<AnyExpr>> {
    vec![Box::new(self.expr.clone())]
  }

  fn create_operation(&self, graph: &mut Graph, children: &[Rc<Operation>], id_gen: &mut FnMut() -> String) -> Result<Operation, Status> {
    let mut nd = try!(graph.new_operation("Neg", &id_gen()));
    nd.add_input(Port {operation: &children[0], index: 0});
    nd.finish()
  }
}

////////////////////////

/// Expression for a variable.
#[derive(Debug)]
pub struct Variable<T: TensorType> {
  shape: Vec<u64>,
  name: String,
  phantom: PhantomData<T>,
}

impl<T: TensorType> Variable<T> {
  fn new(shape: &[u64], name: &str) -> Self {
    Variable {
      shape: Vec::from(shape),
      name: name.to_string(),
      phantom: PhantomData,
    }
  }

  /// Creates an `Expr` for a variable.
  pub fn new_expr(shape: &[u64], name: &str) -> Expr<T> {
    Expr {
      expr: Rc::new(Variable::new(shape, name))
    }
  }
}

impl<T: TensorType> Display for Variable<T> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{}", self.name)
  }
}

impl<T: TensorType> ExprImpl<T> for Variable<T> {
  fn op_level(&self) -> OpLevel {
    OpLevel::Atom
  }

  fn children(&self) -> Vec<Box<AnyExpr>> {
    vec![]
  }

  fn create_operation(&self, graph: &mut Graph, _children: &[Rc<Operation>], _id_gen: &mut FnMut() -> String) -> Result<Operation, Status> {
    let mut nd = try!(graph.new_operation("Variable", &self.name));
    nd.set_attr_type("dtype", DataType::Float).unwrap();
    nd.set_attr_shape("shape", &vec![]).unwrap();
    nd.finish()
  }
}

////////////////////////

/// Expression for a placeholder.
#[derive(Debug)]
pub struct Placeholder<T: TensorType> {
  shape: Vec<u64>,
  name: String,
  phantom: PhantomData<T>,
}

impl<T: TensorType> Placeholder<T> {
  fn new(shape: &[u64], name: &str) -> Self {
    Placeholder {
      shape: Vec::from(shape),
      name: name.to_string(),
      phantom: PhantomData,
    }
  }

  /// Creates an `Expr` for a placeholder.
  pub fn new_expr(shape: &[u64], name: &str) -> Expr<T> {
    Expr {
      expr: Rc::new(Placeholder::new(shape, name))
    }
  }
}

impl<T: TensorType> Display for Placeholder<T> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{}", self.name)
  }
}

impl<T: TensorType> ExprImpl<T> for Placeholder<T> {
  fn op_level(&self) -> OpLevel {
    OpLevel::Atom
  }

  fn children(&self) -> Vec<Box<AnyExpr>> {
    vec![]
  }

  fn create_operation(&self, graph: &mut Graph, _children: &[Rc<Operation>], _id_gen: &mut FnMut() -> String) -> Result<Operation, Status> {
    let mut nd = try!(graph.new_operation("Placeholder", &self.name));
    nd.set_attr_type("dtype", DataType::Float).unwrap();
    nd.set_attr_shape("shape", &vec![]).unwrap();
    nd.finish()
  }
}

////////////////////////

/// Expression that assigns a value to a variable.
#[derive(Debug)]
pub struct Assign<T: TensorType> {
  variable: Expr<T>,
  value: Expr<T>,
}

impl<T: TensorType> Assign<T> {
  fn new(variable: Expr<T>, value: Expr<T>) -> Self {
    Assign {
      variable: variable,
      value: value,
    }
  }

  /// Creates an expression that assigns `value` to `variable`.
  pub fn new_expr(variable: Expr<T>, value: Expr<T>) -> Expr<T> {
    Expr {
      expr: Rc::new(Assign::new(variable, value))
    }
  }
}

impl<T: TensorType> Display for Assign<T> {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{} = {}", self.variable, self.value)
  }
}

impl<T: TensorType> ExprImpl<T> for Assign<T> {
  fn op_level(&self) -> OpLevel {
    OpLevel::Assign
  }

  fn children(&self) -> Vec<Box<AnyExpr>> {
    vec![Box::new(self.variable.clone()), Box::new(self.value.clone())]
  }

  fn create_operation(&self, graph: &mut Graph, children: &[Rc<Operation>], id_gen: &mut FnMut() -> String) -> Result<Operation, Status> {
    let mut nd = try!(graph.new_operation("Assign", &id_gen()));
    nd.add_input(Port {operation: &children[0], index: 0});
    nd.add_input(Port {operation: &children[1], index: 0});
    nd.finish()
  }
}

////////////////////////

// TODO: See if we can make this private.
/// An `AnyExpr` is just an `Expr<T>` for some unknown `T`.
/// Clients *should not* implement this.
pub trait AnyExpr {
  /// Returns a pointer usable as a map key which identifies this expression.
  fn key(&self) -> *const ();

  /// Returns the child expressions.
  ///
  /// For example, the child expressions of `x + y` would be `x` and `y`.
  fn children(&self) -> Vec<Box<AnyExpr>>; // TODO: return an iterator

  /// Creates an operation for the expression.
  ///
  /// The implementation must use the operations in the `children` parameter
  /// rather than creating child operations itself.
  fn create_operation(&self, graph: &mut Graph, children: &[Rc<Operation>], id_gen: &mut FnMut() -> String) -> Result<Operation, Status>;

  /// Returns a boxed clone.
  ///
  /// This is used rather than the `Clone` trait because that would prevent
  /// `AnyExpr` values from being used as trait objects.
  fn clone_box(&self) -> Box<AnyExpr>;
}

impl<T: TensorType> AnyExpr for Expr<T> {
  fn key(&self) -> *const () {
    self.expr.as_ref() as *const ExprImpl<T> as *const ()
  }

  fn children(&self) -> Vec<Box<AnyExpr>> {
    self.expr.children()
  }

  fn create_operation(&self, graph: &mut Graph, children: &[Rc<Operation>], id_gen: &mut FnMut() -> String) -> Result<Operation, Status> {
    self.expr.create_operation(graph, children, id_gen)
  }

  fn clone_box(&self) -> Box<AnyExpr> {
    Box::new(self.clone())
  }
}

struct Key(Box<AnyExpr>);

impl PartialEq for Key {
  fn eq(&self, other: &Key) -> bool {
    self.0.key() == other.0.key()
  }
}

impl Eq for Key {
}

impl Hash for Key {
  fn hash<H>(&self, state: &mut H) where H: Hasher {
    state.write_isize(self.0.key() as isize)
  }
}

/// A `Compiler` compiles `Expr`s to `Operation`s.
pub struct Compiler<'l> {
  graph: &'l mut Graph,
  operations: HashMap<Key, Rc<Operation>>,
  next_id: i32,
}

impl<'l> Compiler<'l> {
  /// Creates a compiler for the given graph.
  pub fn new(graph: &'l mut Graph) -> Self {
    Compiler {
      graph: graph,
      operations: HashMap::new(),
      next_id: 0,
    }
  }

  /// Compiles the expression.
  pub fn compile<T: TensorType>(&mut self, expr: Expr<T>) -> Result<Rc<Operation>, Status> {
    self.compile_any(Box::new(expr))
  }

  /// Compiles the expression.
  pub fn compile_any(&mut self, expr: Box<AnyExpr>) -> Result<Rc<Operation>, Status> {
    let mut child_operations = vec![];
    for child in expr.children() {
      let key = Key(child.clone_box());
      // The result is mapped separately from the match statement below to avoid
      // reference lifetime isues.
      let value = self.operations.get(&key).map(|v| v.clone());
      child_operations.push(match value {
        Some(v) => v,
        None => try!(self.compile_any(child)),
      });
    }
    let mut next_id = self.next_id;
    let result = expr.create_operation(self.graph, &child_operations, &mut || {
      let id = format!("operation_{}", next_id);
      next_id += 1;
      id
    });
    self.next_id = next_id;
    let operation = Rc::new(try!(result));
    self.operations.insert(Key(expr), operation.clone());
    Ok(operation)
  }
}

////////////////////////

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::Graph;

  #[test]
  fn test_display() {
    assert_eq!("1 + 2 + 3", format!("{}", (Expr::from(1) + 2) + 3));
    assert_eq!("1 + 2 + 3", format!("{}", Expr::from(1) + (Expr::from(2) + 3)));
    assert_eq!("1 + 2 - 3", format!("{}", (Expr::from(1) + 2) - 3));
    assert_eq!("1 - (2 + 3)", format!("{}", Expr::from(1) - (Expr::from(2) + 3)));

    assert_eq!("(1 + 2) * 3", format!("{}", (Expr::from(1) + 2) * 3));
    assert_eq!("1 * (2 + 3)", format!("{}", Expr::from(1) * (Expr::from(2) + 3)));
    assert_eq!("1 * 2 * 3", format!("{}", (Expr::from(1) * 2) * 3));
    assert_eq!("1 * 2 * 3", format!("{}", Expr::from(1) * (Expr::from(2) * 3)));

    assert_eq!("(1 + 2) / 3", format!("{}", (Expr::from(1) + 2) / 3));
    assert_eq!("1 / (2 + 3)", format!("{}", Expr::from(1) / (Expr::from(2) + 3)));
    assert_eq!("1 * 2 / 3", format!("{}", (Expr::from(1) * 2) / 3));
    assert_eq!("1 / (2 * 3)", format!("{}", Expr::from(1) / (Expr::from(2) * 3)));

    assert_eq!("(1 + 2) % 3", format!("{}", (Expr::from(1) + 2) % 3));
    assert_eq!("1 % (2 + 3)", format!("{}", Expr::from(1) % (Expr::from(2) + 3)));
    assert_eq!("1 * 2 % 3", format!("{}", (Expr::from(1) * 2) % 3));
    assert_eq!("1 % (2 * 3)", format!("{}", Expr::from(1) % (Expr::from(2) * 3)));

    assert_eq!("-1", format!("{}", -Expr::from(1)));
    assert_eq!("-(-1)", format!("{}", -(-Expr::from(1))));
    assert_eq!("-(1 + 2)", format!("{}", -(Expr::from(1) + 2)));

    assert_eq!("x", format!("{}", <Variable<f32>>::new(&vec![2, 3], "x")));

    assert_eq!("x", format!("{}", <Placeholder<f32>>::new(&vec![2, 3], "x")));

    assert_eq!("x = 1 + 2", format!("{}", Assign::new(
      <Placeholder<f32>>::new_expr(&vec![2, 3], "x"), Expr::from(1.0f32) + 2.0f32)));
  }

  #[test]
  fn test_compile() {
    let mut g = Graph::new();
    let w = <Variable<f32>>::new_expr(&vec![2, 3], "w");
    let x = <Placeholder<f32>>::new_expr(&vec![2, 3], "x");
    let mut compiler = Compiler::new(&mut g);
    compiler.compile(x * w.clone() / w.clone() % w.clone() + w.clone() - w.clone()).unwrap();
    let one = Expr::from(1.0f32);
    compiler.compile(Assign::new_expr(w, one)).unwrap();
  }
}
