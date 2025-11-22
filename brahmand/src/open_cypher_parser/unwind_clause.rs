use nom::{
    bytes::complete::tag_no_case,
    IResult,
    Parser,
};

use super::{
    ast::UnwindClause,
    common::ws,
    errors::OpenCypherParsingError,
    expression::{parse_expression, parse_identifier},
};

fn parse_unwind_internal(input: &str) -> IResult<&str, UnwindClause> {
    let (input, _) = ws(tag_no_case("UNWIND")).parse(input)?;
    let (input, expression) = parse_expression.parse(input)?;
    let (input, _) = ws(tag_no_case("AS")).parse(input)?;
    let (input, alias) = ws(parse_identifier).parse(input)?;

    Ok((
        input,
        UnwindClause {
            expression,
            alias,
        },
    ))
}

/// Parse UNWIND clause: UNWIND <expression> AS <alias>
/// Example: UNWIND [1, 2, 3] AS x
pub fn parse_unwind_clause(input: &str) -> IResult<&str, UnwindClause, OpenCypherParsingError> {
    parse_unwind_internal(input).map_err(|e| match e {
        nom::Err::Incomplete(needed) => nom::Err::Incomplete(needed),
        nom::Err::Error(err) => nom::Err::Failure(OpenCypherParsingError::from(err)),
        nom::Err::Failure(err) => nom::Err::Failure(OpenCypherParsingError::from(err)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::open_cypher_parser::ast::Expression;

    #[test]
    fn test_parse_unwind_clause_with_list() {
        let input = "UNWIND [1, 2, 3] AS x";
        let result = parse_unwind_clause(input);
        assert!(result.is_ok());
        let (remaining, unwind) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(unwind.alias, "x");
        // The expression should be a list
        assert!(matches!(unwind.expression, Expression::List(_)));
    }

    #[test]
    fn test_parse_unwind_clause_with_variable() {
        let input = "UNWIND items AS item";
        let result = parse_unwind_clause(input);
        assert!(result.is_ok());
        let (remaining, unwind) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(unwind.alias, "item");
        assert!(matches!(unwind.expression, Expression::Variable(_)));
    }

    #[test]
    fn test_parse_unwind_clause_case_insensitive() {
        let input = "unwind [1, 2] as n";
        let result = parse_unwind_clause(input);
        assert!(result.is_ok());
        let (_, unwind) = result.unwrap();
        assert_eq!(unwind.alias, "n");
    }
}
