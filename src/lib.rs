use nom::bytes::complete::{take_till, take_while};
use nom::IResult;
use std::error::Error;

/// Convert a string to a convenient view for natural sorting.
/// E.g., output string may be stored into database for ordering by.
///
/// Example:
/// ```
/// use naturalize::to_natural;
///
/// let nat = to_natural("abc123def").unwrap();
/// assert_eq!(nat, "abc0000000123def");
/// ```
pub fn to_natural(src: &str) -> Result<String, Box<dyn Error>> {
    let (_, natur) = run_parser(&src).unwrap();
    Ok(natur)
}

/// Run parser.
fn run_parser(input: &str) -> IResult<&str, String> {
    let mut natur = String::new();
    let (input, natur) = text_num(input, &mut natur)?;

    Ok((input, (*natur).to_string()))
}

/// Комбинатор парсеров для выципления пары текста и числа с дополнением числа нулями
/// Parser combinator for taking of pair with text and number. Append number with zeroes.
fn text_num<'a>(input: &'a str, natur: &mut String) -> IResult<&'a str, String> {
    let (input, prefix) = take_till(is_digit)(input)?;
    natur.push_str(&prefix);
    let (input, prefix) = take_while(is_digit)(input)?;
    if prefix.is_empty() {
        Ok((input, (*natur).to_string()))
    } else {
        let prefix = format!("{:0>10}", prefix);
        natur.push_str(&prefix);
        text_num(input, natur)
    }
}

/// Check that character is digit
fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

#[cfg(test)]
mod tests {
    use super::to_natural;

    #[test]
    fn test_empty() {
        assert_eq!(to_natural("").unwrap(), "".to_string());
    }

    #[test]
    fn test_num() {
        assert_eq!(to_natural("1020").unwrap(), "0000001020".to_string());
    }

    #[test]
    fn test_big_num() {
        assert_eq!(
            to_natural("102030405060708090").unwrap(),
            "102030405060708090".to_string()
        );
    }

    #[test]
    fn test_text() {
        assert_eq!(to_natural("Hello").unwrap(), "Hello".to_string());
    }

    #[test]
    fn test_num_text() {
        assert_eq!(
            to_natural("10 apples").unwrap(),
            "0000000010 apples".to_string()
        );
    }

    #[test]
    fn test_text_num() {
        assert_eq!(
            to_natural("apples 10").unwrap(),
            "apples 0000000010".to_string()
        );
    }

    #[test]
    fn test_ip1() {
        assert_eq!(
            to_natural("172.29.21.151").unwrap(),
            "0000000172.0000000029.0000000021.0000000151".to_string()
        );
    }

    #[test]
    fn test_ip2() {
        assert_eq!(
            to_natural("IP = 172.29.21.151").unwrap(),
            "IP = 0000000172.0000000029.0000000021.0000000151".to_string()
        );
    }
}