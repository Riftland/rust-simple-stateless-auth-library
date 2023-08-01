#[derive(Debug)]
enum Type {
    Days,
    Hours,
    Minutes,
    Seconds,
    Milliseconds,
}

impl Type {
    fn days_to_ms(value: i32) -> i32 {
        value * 24 * 60 * 60 * 1000
    }

    fn hours_to_ms(value: i32) -> i32 {
        value * 60 * 60 * 1000
    }

    fn minutes_to_ms(value: i32) -> i32 {
        value * 60 * 1000
    }

    fn seconds_to_ms(value: i32) -> i32 {
        value * 1000
    }
}

#[derive(Debug, PartialEq)]
enum ValueError {
    ZeroValueError,
    NegativeValueError
}

fn parse_exp_time(value: i32, time: Type) -> Result<i32, ValueError> {
    if value == 0 {
        return Err(ValueError::ZeroValueError);
    }

    if value < 0 {
        return Err(ValueError::NegativeValueError);
    }

    Ok(match time {
        Type::Days => Type::days_to_ms(value),
        Type::Hours => Type::hours_to_ms(value),
        Type::Minutes => Type::minutes_to_ms(value),
        Type::Seconds => Type::seconds_to_ms(value),
        Type::Milliseconds => value,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_days_to_ms() {
        let expected_ms = 864_000_000;
        let value = 10;
        let time = Type::Days;

        let milliseconds = parse_exp_time(value, time).unwrap();

        assert_eq!(milliseconds, expected_ms);
    }

    #[test]
    fn parse_hours_to_ms() {
        let expected_ms = 36_000_000;
        let value = 10;
        let time = Type::Hours;

        let milliseconds = parse_exp_time(value, time).unwrap();

        assert_eq!(milliseconds, expected_ms);
    }

    #[test]
    fn parse_minutes_to_ms() {
        let expected_ms = 1_560_000;
        let value = 26;
        let time = Type::Minutes;

        let milliseconds = parse_exp_time(value, time).unwrap();

        assert_eq!(milliseconds, expected_ms);
    }

    #[test]
    fn parse_seconds_to_ms() {
        let expected_ms = 300_000;
        let value = 300;
        let time = Type::Seconds;

        let milliseconds = parse_exp_time(value, time).unwrap();

        assert_eq!(milliseconds, expected_ms);
    }

    #[test]
    fn parse_ms_to_ms() {
        let value = 1200;
        let time = Type::Milliseconds;

        let milliseconds = parse_exp_time(value, time).unwrap();

        assert_eq!(milliseconds, value);
    }

    #[test]
    fn zero_value_to_ms() {
        let value = 0;
        let time = Type::Seconds;

        let milliseconds = parse_exp_time(value, time).unwrap_err();

        assert_eq!(milliseconds, ValueError::ZeroValueError);
    }

    #[test]
    fn negative_value_to_ms() {
        let value = -100;
        let time = Type::Minutes;

        let milliseconds = parse_exp_time(value, time).unwrap_err();

        assert_eq!(milliseconds, ValueError::NegativeValueError);
    }
}