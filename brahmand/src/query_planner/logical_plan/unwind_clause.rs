use std::sync::Arc;

use crate::open_cypher_parser::ast::UnwindClause;
use crate::query_planner::{
    logical_expr::ColumnAlias,
    logical_plan::LogicalPlan,
};

use super::Unwind;

pub fn evaluate_unwind_clause(
    unwind_clause: &UnwindClause,
    input: Arc<LogicalPlan>,
) -> Arc<LogicalPlan> {
    Arc::new(LogicalPlan::Unwind(Unwind {
        input,
        expression: unwind_clause.expression.clone().into(),
        alias: ColumnAlias(unwind_clause.alias.to_string()),
    }))
}
