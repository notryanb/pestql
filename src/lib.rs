extern crate pest;
#[macro_use] extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "sql.pest"]
pub struct SQLParser;

pub fn is_valid_sql(query: &str) -> bool {
    SQLParser::parse(Rule::sql, query).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_select() {
        let statement = "select 1 from test;";
        assert_eq!(true, is_valid_sql(statement))
    }

    #[test]
    fn parses_select_star() {
        let statement = "select * from test;";
        assert_eq!(true, is_valid_sql(statement))
    }

    #[test]
    fn parses_select_two_columns() {
        let statement = "select something,another_one from test;";
        assert_eq!(true, is_valid_sql(statement))
    }

    #[test]
    fn parses_select_many_columns() {
        let statement = "select something,another_one,yet_another,so_many_columns from test;";
        assert_eq!(true, is_valid_sql(statement))
    }

        #[test]
    fn parses_select_many_columns_with_spaces() {
        let statement = "select something, another_one, yet_another, so_many_columns from test;";
        assert_eq!(true, is_valid_sql(statement))
    }

        #[test]
    fn error_when_trailing_comma_in_select() {
        let statement = "select something,another_one,yet_another,so_many_columns, from test;";
        assert_eq!(false, is_valid_sql(statement))
    }
}
