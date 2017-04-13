use syntax::ast;
use syntax::codemap::DUMMY_SP;
use syntax::symbol;
use syntax::tokenstream;

use aster::AstBuilder;

#[test]
fn test_named() {
    let builder = AstBuilder::new();

    assert_eq!(
        builder.struct_field("x").ty().isize(),
        ast::StructField {
            ident: Some(builder.id("x")),
            vis: ast::Visibility::Inherited,
            id: ast::DUMMY_NODE_ID,
            ty: builder.ty().isize(),
            attrs: vec![],
            span: DUMMY_SP
        }
    );
}

#[test]
fn test_unnamed() {
    let builder = AstBuilder::new();

    assert_eq!(
        builder.tuple_field().ty().isize(),
        ast::StructField {
            ident: None,
            vis: ast::Visibility::Inherited,
            id: ast::DUMMY_NODE_ID,
            ty: builder.ty().isize(),
            attrs: vec![],
            span: DUMMY_SP
        }
    );
}

#[test]
fn test_attrs() {
    let builder = AstBuilder::new();

    assert_eq!(
        ast::StructField {
            ident: Some(builder.id("x")),
            vis: ast::Visibility::Inherited,
            id: ast::DUMMY_NODE_ID,
            ty: builder.ty().isize(),
            span: DUMMY_SP,
            attrs: vec![
                ast::Attribute {
                    id: ast::AttrId(0),
                    style: ast::AttrStyle::Outer,
                    path: ast::Path {
                        span: DUMMY_SP,
                        segments: vec![ast::PathSegment{
                            identifier: symbol::Ident::from_str("doc"),
                            span: DUMMY_SP,
                            parameters: None,
                        }],
                    },
                    tokens: ast::MetaItemKind::NameValue((*builder.lit().str("/// doc string")).clone()).tokens(DUMMY_SP),
                    is_sugared_doc: true,
                    span: DUMMY_SP,
                },
                ast::Attribute {
                    id: ast::AttrId(1),
                    style: ast::AttrStyle::Outer,
                    path: ast::Path {
                        span: DUMMY_SP,
                        segments: vec![ast::PathSegment{
                            identifier: symbol::Ident::from_str("automatically_derived"),
                            span: DUMMY_SP,
                            parameters: None,
                        }],
                    },
                    tokens: tokenstream::TokenStream::empty(),
                    is_sugared_doc: false,
                    span: DUMMY_SP,
                },
            ],
        },
        builder.struct_field("x")
            .attr().doc("/// doc string")
            .attr().automatically_derived()
            .ty().isize()
    );
}
