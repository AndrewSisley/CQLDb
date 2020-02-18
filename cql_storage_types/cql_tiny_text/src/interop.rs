use std::{ fmt };
use std::convert::TryFrom;

use crate::{ TinyText, errors::ValueTooLargeError, CONTENT_SIZE };

impl From<TinyText> for String {
    fn from(text: TinyText) -> Self {
        text.0
    }
}

/// Attempts to convert the given string to TinyText. Errors if the length of the String excedes 1020 bytes.
///
/// # Errors
/// Will return a [ValueTooLargeError](errors/struct.ValueTooLargeError.html) if the given string is larger than 1020 bytes.
///
/// # Examples
/// ```
/// # use cql_tiny_text::{ TinyText, errors::ValueTooLargeError };
/// use std::convert::TryFrom;
///
/// let small_string = "s".repeat(1020); // 1 byte per char
/// assert_eq!("s".repeat(1020), String::from(TinyText::try_from(small_string).unwrap()));
///
/// let big_string = "s".repeat(1021);
/// assert_eq!(Err(ValueTooLargeError), TinyText::try_from(big_string));
///
/// let small_utf8_string = "س".repeat(510); // 2 bytes per char
/// assert_eq!("س".repeat(510), String::from(TinyText::try_from(small_utf8_string).unwrap()));
///
/// let big_utf8_string = "س".repeat(511);
/// assert_eq!(Err(ValueTooLargeError), TinyText::try_from(big_utf8_string));
/// ```
impl TryFrom<String> for TinyText {
    type Error = ValueTooLargeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > CONTENT_SIZE {
            Err(ValueTooLargeError)
        } else {
            Ok(
                TinyText(value)
            )
        }
    }
}

/// Attempts to convert the given &str to TinyText. Errors if the length of the &str excedes 1020 bytes.
///
/// # Examples
/// ```
/// # use cql_tiny_text::{ TinyText, errors::ValueTooLargeError };
/// use std::convert::TryFrom;
///
/// let small_string = "s".repeat(1020); // 1 byte per char
/// assert_eq!("s".repeat(1020), String::from(TinyText::try_from(small_string).unwrap()));
///
/// let big_string = "s".repeat(1021);
/// assert_eq!(Err(ValueTooLargeError), TinyText::try_from(big_string));
///
/// let small_utf8_string = "س".repeat(510); // 2 bytes per char
/// assert_eq!("س".repeat(510), String::from(TinyText::try_from(small_utf8_string).unwrap()));
///
/// let big_utf8_string = "س".repeat(511);
/// assert_eq!(Err(ValueTooLargeError), TinyText::try_from(big_utf8_string));
/// ```
impl TryFrom<&str> for TinyText {
    type Error = ValueTooLargeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TinyText::try_from(value.to_string())
    }
}

impl fmt::Display for TinyText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
