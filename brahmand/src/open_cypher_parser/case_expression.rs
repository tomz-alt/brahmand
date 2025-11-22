use crate::open_cypher_parser::ast::{CaseExpression, Expression};
use crate::open_cypher_parser::common::ws;
use crate::open_cypher_parser::expression::parse_expression;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::combinator::{cut, map, opt};
use nom::error::ErrorKind;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::{IResult, Parser};

/// Parse a CASE expression
///
/// Supports two forms:
/// 1. Searched CASE: CASE WHEN condition THEN result ... ELSE default END
/// 2. Simple CASE: CASE expr WHEN value THEN result ... ELSE default END
pub fn parse_case_expression(input: &'_ str) -> IResult<&'_ str, Expression<'_>> {
    // CASE keyword
    let (input, _) = ws(tag_no_case("CASE")).parse(input)?;

    // Try to parse optional test expression (for simple CASE)
    // We need to differentiate between:
    //   CASE WHEN ... (no test expr)
    //   CASE x WHEN ... (test expr = x)
    //
    // Strategy: Look ahead for WHEN. If we see expression before WHEN, it's simple CASE
    let (input, test_expr) = opt(|i| {
        // Try to parse an expression, but only if it's NOT immediately followed by WHEN
        let (i2, expr) = parse_expression(i)?;
        // Check if next non-whitespace is WHEN
        let (i3, _) = multispace0(i2)?;

        // If we see WHEN, this expression is actually the condition, not the test
        // So we return None by failing this optional parser
        if i3.trim_start().to_lowercase().starts_with("when") {
            // This is actually a searched CASE, backtrack
            return Err(nom::Err::Error(nom::error::Error::new(i, ErrorKind::Tag)));
        }

        Ok((i2, Box::new(expr)))
    })
    .parse(input)?;

    // Parse one or more WHEN-THEN pairs
    let (input, when_then_pairs) = many1(|i| {
        let (i, _) = ws(tag_no_case("WHEN")).parse(i)?;
        let (i, when_expr) = cut(parse_expression).parse(i)?;
        let (i, _) = ws(tag_no_case("THEN")).parse(i)?;
        let (i, then_expr) = cut(parse_expression).parse(i)?;
        Ok((i, (when_expr, then_expr)))
    })
    .parse(input)?;

    // Optional ELSE clause
    let (input, else_expr) =
        opt(preceded(ws(tag_no_case("ELSE")), map(parse_expression, Box::new)))
            .parse(input)?;

    // END keyword
    let (input, _) = ws(tag_no_case("END")).parse(input)?;

    Ok((
        input,
        Expression::CaseExp(CaseExpression {
            test_expr,
            when_then_pairs,
            else_expr,
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_searched_case() {
        let input = "CASE WHEN x > 10 THEN 'high' WHEN x > 5 THEN 'medium' ELSE 'low' END";
        let result = parse_case_expression(input);
        assert!(result.is_ok());

        if let Ok((_, Expression::CaseExp(case))) = result {
            assert!(case.test_expr.is_none()); // Searched CASE
            assert_eq!(case.when_then_pairs.len(), 2);
            assert!(case.else_expr.is_some());
        } else {
            panic!("Failed to parse searched CASE");
        }
    }

    #[test]
    fn test_parse_simple_case() {
        let input = "CASE status WHEN 'active' THEN 1 WHEN 'inactive' THEN 0 ELSE -1 END";
        let result = parse_case_expression(input);
        assert!(result.is_ok());

        if let Ok((_, Expression::CaseExp(case))) = result {
            assert!(case.test_expr.is_some()); // Simple CASE
            assert_eq!(case.when_then_pairs.len(), 2);
            assert!(case.else_expr.is_some());
        } else {
            panic!("Failed to parse simple CASE");
        }
    }

    #[test]
    fn test_parse_case_without_else() {
        let input = "CASE WHEN x = 1 THEN 'one' WHEN x = 2 THEN 'two' END";
        let result = parse_case_expression(input);
        assert!(result.is_ok());

        if let Ok((_, Expression::CaseExp(case))) = result {
            assert!(case.test_expr.is_none());
            assert_eq!(case.when_then_pairs.len(), 2);
            assert!(case.else_expr.is_none());
        } else {
            panic!("Failed to parse CASE without ELSE");
        }
    }

    #[test]
    fn test_parse_case_single_when() {
        let input = "CASE WHEN active THEN 'yes' ELSE 'no' END";
        let result = parse_case_expression(input);
        assert!(result.is_ok());

        if let Ok((_, Expression::CaseExp(case))) = result {
            assert_eq!(case.when_then_pairs.len(), 1);
            assert!(case.else_expr.is_some());
        } else {
            panic!("Failed to parse CASE with single WHEN");
        }
    }
}
