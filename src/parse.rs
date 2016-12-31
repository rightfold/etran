include!(concat!(env!("OUT_DIR"), "/grammar.rs"));

#[cfg(test)]
mod tests {
    use ast::{Expression, Step};
    use super::step;
    use value::{Level, Value};

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
