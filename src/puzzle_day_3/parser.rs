use std;
use nom::{digit, space, IResult};


named!(numeric_string<&str>,
    map_res!(
        digit,
        std::str::from_utf8
    )
);

named!(u32_digit<u32>,
    map_res!(
        numeric_string,
        std::str::FromStr::from_str
    )
);

named!(three_u32_digits<(u32, u32, u32)>,
    chain!(
        space? ~
        side_1: u32_digit ~
        space? ~
        side_2: u32_digit ~
        space? ~
        side_3: u32_digit
        ,
        || (side_1, side_2, side_3)
    )
);


pub fn sides_of_triangle(line: &str) -> (u32, u32, u32) {
    match three_u32_digits(line.as_bytes()) {
        IResult::Done(_, (a, b, c)) => (a, b, c),
        _ => panic!("could not extract three u32 digits from line '{}'", line),
    }
}


#[cfg(test)]
mod tests {
    use hamcrest::prelude::*;
    use nom::{IResult, Needed};

    #[test]
    fn successful_parsing_test() {
        let parsing_result = super::three_u32_digits("111 222 333".as_bytes());

        assert_that!(parsing_result, is(equal_to(IResult::Done(&b""[..], (111, 222, 333) ))));
    }

    #[test]
    fn test_resulting_in_error() {
        let parsing_result = super::three_u32_digits("abc".as_bytes());

        let test_passed = match parsing_result {
            IResult::Error(_) => true,
            _ => false,
        };

        assert_that!(test_passed, is(equal_to(true)));
    }

    #[test]
    fn test_resulting_in_incompleteness() {
        let parsing_result = super::three_u32_digits("111 222".as_bytes());

        assert_that!(parsing_result, is(equal_to(IResult::Incomplete(Needed::Unknown))));
    }
}
