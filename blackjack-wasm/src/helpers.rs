use crate::constants::Value;

pub(crate) fn get_value(value: Value) -> (u16, Option<u16>) {
    let val = match value {
        Value::Two => 2,
        Value::Three => 3,
        Value::Four => 4,
        Value::Five => 5,
        Value::Six => 6,
        Value::Seven => 7,
        Value::Eight => 8,
        Value::Nine => 9,
        Value::Ace => return (10, Some(1)),
        _ => 10,
    };

    return (val, None);
}
