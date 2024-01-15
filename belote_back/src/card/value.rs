use core::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Value {
    Number(u8),
    King,
    Queen,
    Jack,
    Ace,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Value::Number(n) => n.to_string(),
            Value::King => "K".to_string(),
            Value::Queen => "Q".to_string(),
            Value::Jack => "J".to_string(),
            Value::Ace => "A".to_string(),
        };
        write!(f, "{}", value)
    }
}

impl Value {
    pub fn get_value(&self) -> i32 {
        match self {
            Value::Number(n) => match n {
                10 => 10,
                _ => 0,
            },
            Value::King => 4,
            Value::Queen => 3,
            Value::Jack => 2,
            Value::Ace => 11,
        }
    }
}

impl std::ops::Add for Value {
    type Output = i32;

    fn add(self, other: Value) -> Self::Output {
        self.get_value() + other.get_value()
    }
}
