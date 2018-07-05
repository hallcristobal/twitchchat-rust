use chrono;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        IoError(err: ::std::io::Error) {
            from()
        }
        ParseNumError(err: ::std::num::ParseIntError) {
            from()
        }
        ChronoParseError(err: chrono::ParseError) {
            from()
        }
        StringParseError(err: ::std::string::ParseError) {
            from()
        }
        Custom(err: String) {
            from()
            description(err)
        }
    }
}
