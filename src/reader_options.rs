/// Options object for the terminal reader.
///
/// This object is used to configure the behaviour of the terminal reader.
pub struct ReaderOptions {
    /// Echo input characters
    pub echo: bool,
}

impl ReaderOptions {
    /// Generate an options object with the default properties.
    pub fn default() -> ReaderOptions {
        ReaderOptions {
            echo: false,
        }
    }
}
