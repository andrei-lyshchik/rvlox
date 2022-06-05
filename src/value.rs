#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Double(f64),
}

macro_rules! binary_operator {
    (
        $sel:ident, $name:ident, $op: tt
    ) => {
        pub fn $name(&$sel, other: &Value) -> Value {
            match ($sel, &other) {
                (Value::Double(l), Value::Double(r)) => Value::Double(l $op r)
            }
        }
    }
}

impl Value {
    pub fn negate(&self) -> Value {
        match self {
            Value::Double(d) => Value::Double(-d),
        }
    }

    binary_operator!(self, add, -);

    binary_operator!(self, subtract, -);

    binary_operator!(self, multiply, *);

    binary_operator!(self, divide, /);
}
