use ast::Type;
use value::{Level, Value};

pub fn serialize(value: &Value, into: &mut String) {
    match *value {
        Value::Boolean(b) => {
            into.push(if b { 't' } else { 'f' });
        },
        Value::Level(l) => {
            into.push(match l {
                Level::Debug    => '7',
                Level::Info     => '6',
                Level::Notice   => '5',
                Level::Warning  => '4',
                Level::Error    => '3',
                Level::Critical => '2',
                Level::Alert    => '1',
            });
        },
    }
}

pub fn deserialize<'a>(type_: &Type, from: &'a str) -> Option<(&'a str, Value)> {
    match *type_ {
        Type::Boolean => deserialize_boolean(from),
    }
}

fn deserialize_boolean(from: &str) -> Option<(&str, Value)> {
    match from.split_at(1) {
        ("t", r) => Some((r, Value::Boolean(true))),
        ("f", r) => Some((r, Value::Boolean(false))),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use ast::Type;
    use super::*;
    use value::Value;

    fn test(type_: &Type, value: Value) {
        let mut buf = String::new();
        serialize(&value, &mut buf);
        assert_eq!(Some(("", value)), deserialize(type_, &buf));
    }

    #[test]
    fn test_boolean() {
        test(&Type::Boolean, Value::Boolean(true));
        test(&Type::Boolean, Value::Boolean(false));
    }
}
