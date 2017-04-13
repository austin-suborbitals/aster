use syntax::ast;
use syntax::symbol;
use syntax::codemap::DUMMY_SP;

use aster::AstBuilder;

#[test]
fn test_doc() {
    let builder = AstBuilder::new();
    assert_eq!(
        builder.attr().doc("/// doc string"),
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
        }
    );
}
