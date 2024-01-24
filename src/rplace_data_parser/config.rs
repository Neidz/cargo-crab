pub enum OnError {
    Stop,
    Print,
    Nothing,
}

pub struct ParserConfig {
    pub verbose: bool,
    pub on_error: OnError,
    pub output_dir: String,
    pub save_interval_seconds: u32,
}

impl ParserConfig {
    pub fn new(
        verbose: bool,
        output_dir: String,
        on_error: OnError,
        save_interval_seconds: u32,
    ) -> ParserConfig {
        ParserConfig {
            verbose,
            output_dir,
            on_error,
            save_interval_seconds,
        }
    }

    pub fn new_default() -> ParserConfig {
        ParserConfig {
            verbose: true,
            output_dir: String::from("output/output_images"),
            on_error: OnError::Print,
            save_interval_seconds: 10000,
        }
    }
}
