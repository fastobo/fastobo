/// Error type for `FromStr` result.
#[derive(Debug, PartialEq, Fail)]
pub enum ParseError {
    #[fail(display = "error occured")]
    AnyError,
    // RemainingInput { remainer: String },
    // #[fail(display = "invalid character: '{}'", invalid)]
    // InvalidCharacter { invalid: char },
    // #[fail(display = "nom parser error: '{}'", description)]
    // NomError { description: String },
}
