include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

#[cfg(test)]
mod tests {
    use ast::{Definition, Expression, From, Step};
    use super::*;
    use value::{Level, Value};

    #[test]
    fn test_filter_definition() {
        assert_eq!(
            definition("FILTER f AS FROM s AS n;"),
            Ok(Definition::Filter("f", From{stream: "s", name: "n"}, vec![]))
        );
        assert_eq!(
            definition("FILTER f AS FROM s AS n WHERE INFO;"),
            Ok(Definition::Filter("f", From{stream: "s", name: "n"}, vec![Step::Where(Expression::Literal(Value::Level(Level::Info)))]))
        );
    }

    #[test]
    fn test_raise_step() {
        assert_eq!(
            step("RAISE INFO WARNING"),
            Ok(Step::Raise(Level::Info, Expression::Literal(Value::Level(Level::Warning))))
        );
    }

    #[test]
    fn test_where_step() {
        assert_eq!(
            step("WHERE WARNING"),
            Ok(Step::Where(Expression::Literal(Value::Level(Level::Warning))))
        );
    }
}
