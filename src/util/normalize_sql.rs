use std::ops::ControlFlow;

use sqlparser::ast::{Expr, Query, SetExpr, VisitMut, VisitorMut};
use sqlparser::dialect::Dialect;
use sqlparser::parser::Parser;

use crate::util::Result;

struct Normalizer;

impl VisitorMut for Normalizer {
    type Break = ();

    fn post_visit_query(&mut self, query: &mut Query) -> ControlFlow<Self::Break> {
        if let SetExpr::Select(set_query) = query.body.as_mut() {
            // Remove TOP clause from MSSQL queries, since this is a runtime
            // parameter and not part of the view configuration.
            set_query.top = None;
        }
        ControlFlow::Continue(())
    }

    fn post_visit_expr(&mut self, expr: &mut Expr) -> ControlFlow<Self::Break> {
        if let Expr::Identifier(id) = expr {
            // Replace @P<number> SQL parameters with ?, to make it less
            // dependent on numbering order.
            if id.value.starts_with("@P") {
                id.value = "?".to_owned();
            }
        }
        ControlFlow::Continue(())
    }
}

/// Normalize an SQL query to a common format for comparison.
///
/// # Arguments
/// * `dialect` - The SQL dialect to use for parsing the query.
/// * `sql` - The SQL query to normalize.
///
/// # Returns
/// * A normalized version of the SQL query.
pub(crate) fn normalize_sql(dialect: &dyn Dialect, sql: &str) -> Result<String> {
    let parser = Parser::new(dialect);
    let mut parser = match parser.try_with_sql(sql) {
        Ok(parser) => parser,
        Err(e) => {
            return Err(e.into());
        }
    };
    let mut ast = match parser.parse_statement() {
        Ok(ast) => ast,
        Err(e) => {
            return Err(e.into());
        }
    };
    ast.visit(&mut Normalizer);
    Ok(ast.to_string())
}
