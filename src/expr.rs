use crate::token::{Token, LiteralValue};
use std::marker::PhantomData;

pub trait Expr<'a> { 
    fn ast_printer(&self) -> (String, Vec<&'a dyn ExprConstraint<'a>>);
}

pub trait ExprHelpers<'a> {
    fn name(&self) -> &str;
}

pub trait ExprConstraint<'a>: Expr<'a> + ExprHelpers<'a> {}
impl<'a, T: ?Sized> ExprConstraint<'a> for T where T: Expr<'a> + ExprHelpers<'a> {}

macro_rules! expression {
   ($struct:ident<$lt:lifetime> {$( $field:ident:$type:ty ), *}) => {
        pub struct $struct<$lt> {
            $(
                pub $field: $type,
            )*
            pub _phantom: PhantomData<&$lt str>
        }

        /* How to get rid of phantom data?
        impl<$lt> $struct<$lt> {
            pub fn new($($field: $type,)*) -> $struct<$lt> {
                $struct {
                    $(
                        $field: $field, 
                    )*
                    _phantom: PhantomData
                }
            }
        }*/

        impl<$lt> ExprHelpers<$lt> for $struct<$lt> {
            fn name(&self) -> &str {
                "$struct"
            }
        }
    }
}

expression!(BinaryExpr<'a> { left: &'a dyn ExprConstraint<'a>, operator: Token, right: &'a dyn ExprConstraint<'a> });

impl<'a> Expr<'a> for BinaryExpr<'a> {
    fn ast_printer(&self) -> (String, Vec<&'a dyn ExprConstraint<'a>>) {
        (self.operator.lexeme.clone(), vec![self.left, self.right]) 
    }
}

expression!(GroupingExpr<'a> { expression: &'a dyn ExprConstraint<'a> });

impl<'a> Expr<'a> for GroupingExpr<'a> {
    fn ast_printer(&self) -> (String, Vec<&'a dyn ExprConstraint<'a>>) {
        (String::from("group"), vec![self.expression])
    }
}

expression!(LiteralExpr<'a> { value: LiteralValue });

impl<'a> Expr<'a> for LiteralExpr<'a> {
    fn ast_printer(&self) -> (String, Vec<&'a dyn ExprConstraint<'a>>) {
        (self.value.to_string(), vec![])
    }
}

expression!(UnaryExpr<'a> { operator: Token, right: &'a dyn ExprConstraint<'a> });

impl<'a> Expr<'a> for UnaryExpr<'a> {
    fn ast_printer(&self) -> (String, Vec<&'a dyn ExprConstraint<'a>>) {
        (self.operator.lexeme.clone(), vec![self.right])
    }
}
