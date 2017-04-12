#![feature(i128_type)]
#![feature(rustc_private)]

extern crate aster;
extern crate syntax;

#[cfg(feature = "unstable")]
extern crate compiletest_rs as compiletest;

mod test_arm;
mod test_attr;
mod test_expr;
mod test_fn_decl;
mod test_generics;
mod test_item;
mod test_lit;
mod test_pat;
mod test_path;
mod test_stmt;
mod test_struct_field;
mod test_ty;
mod test_ty_param;
mod test_variant;
mod test_variant_data;
mod test_where_predicate;

#[cfg(feature = "unstable")]
mod compile_tests;
