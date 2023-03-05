#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_word_without_path_or_file() {
        let args = Args {
            path: String::new(),
            word: String::new(),
            filetype: None
        };
        let result = args.validate();
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err().to_string(), "error: The following required arguments were not provided:\n    <word>\n\nUSAGE:\n    \n    [OPTIONS] --path <path>\n    \n    [OPTIONS] --word <word>\n");
    }

    #[test]
    fn test_required_word_with_path() {
        let args = Args {
            path: String::from("/path/to/file"),
            word: String::new(),
            filetype: None
        };
        let result = args.validate();
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err().to_string(), "error: The following required arguments were not provided:\n    <word>\n\nUSAGE:\n    \n    [OPTIONS] --path <path>\n    \n    [OPTIONS] --word <word>\n");
    }

    #[test]
    fn test_required_word_with_file() {
        let args = Args {
            path: String::new(),
            word: String::from("test"),
            filetype: None
        };
        let result = args.validate();
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err().to_string(), "error: The following required arguments were not provided:\n    --path <path>\n\nUSAGE:\n    \n    [OPTIONS] --path <path>\n    \n    [OPTIONS] --word <word>\n");

    }

    #[test]
    fn test_valid_args() {
        let args = Args {
            path: String::from("/path/to/file"),
            word: String::from("test"),
            filetype: Some(String::from(".txt"))
        };
        let result = args.validate();
        assert_eq!(result.is_err(), false);
        assert_eq!(result.unwrap(), ());
    }
}
