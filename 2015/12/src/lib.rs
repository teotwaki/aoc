use common::Answer;
use json::{object::Object, parse as parse_json, JsonValue};

type IntType = i32;

fn parse(s: &str) -> Vec<IntType> {
    let s = s
        .chars()
        .map(|c| {
            if c.is_ascii_digit() || c == '-' {
                c
            } else {
                ' '
            }
        })
        .collect::<String>();

    s.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

pub fn step1(s: &str) -> Answer {
    parse(s).iter().sum::<IntType>().into()
}

fn sum_json_object(obj: &Object) -> IntType {
    if obj.iter().any(|(_, val)| val == "red") {
        0
    } else {
        obj.iter().map(|(_, val)| sum_json(val)).sum()
    }
}

fn sum_json(val: &JsonValue) -> IntType {
    match val {
        JsonValue::Number(n) => n.as_fixed_point_i64(0).unwrap() as i32,
        JsonValue::Object(o) => sum_json_object(o),
        JsonValue::Array(vec) => vec.iter().map(sum_json).sum::<IntType>(),
        _ => 0,
    }
}

pub fn step2(s: &str) -> Answer {
    let json = parse_json(s).unwrap();
    sum_json(&json).into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn step1_finds_correct_values() {
        assert_eq!(step1("[1,2,3]"), Answer::Signed(6));
        assert_eq!(step1(r#"{"a":2,"b":4}"#), Answer::Signed(6));

        assert_eq!(step1("[[[3]]]"), Answer::Signed(3));
        assert_eq!(step1(r#"{"a":{"b":4},"c":-1}"#), Answer::Signed(3));

        assert_eq!(step1(r#"{"a":[-1,1]}"#), Answer::Signed(0));
        assert_eq!(step1(r#"[-1,{"a":1}]"#), Answer::Signed(0));

        assert_eq!(step1("[]"), Answer::Signed(0));
        assert_eq!(step1("{}"), Answer::Signed(0));
    }

    #[test]
    fn step2_finds_correct_values() {
        assert_eq!(step2("[1,2,3]"), Answer::Signed(6));
        assert_eq!(step2(r#"[1,{"c":"red","b":2},3]"#), Answer::Signed(4));
        assert_eq!(
            step2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#),
            Answer::Signed(0)
        );
        assert_eq!(step2(r#"[1,"red",5]"#), Answer::Signed(6));
    }
}
