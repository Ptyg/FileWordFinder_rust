#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::env;
    use lib::{find_in_dir, find_in_file, OutResult};
    
    fn get_path() -> PathBuf {
        let mut pathbuf = env::current_dir().unwrap();
        let path_to_dir = ["tests", "test_files"];
        for i in path_to_dir {
            pathbuf.push(i);
        }
        
        pathbuf
    }

    #[test]
    fn test_find_in_file() {
        let mut pathbuf = get_path();
        pathbuf.push("txt_1.txt");
        let file_path = pathbuf.as_path();
        let search_word = String::from("sun");
        let expected = Some(vec![OutResult {
            file_name: file_path.to_string_lossy().into_owned(),
            word: search_word.to_string(),
            line: 0,
        }]);
        
        let result = find_in_file(file_path, &search_word);

        assert_eq!(result.is_some(), expected.is_some());
        assert_eq!(result.unwrap().len(), expected.unwrap().len());
    }

    #[test]
    fn test_find_in_file_path_not_exists() {
        let mut pathbuf = get_path();
        pathbuf.push("txt_not_exists.txt");
        let file_path = pathbuf.as_path();
        let search_word = String::from("sun");
        let result = find_in_file(file_path, &search_word);
        assert!(result.is_none());
    }

    #[test]
    fn test_find_in_file_word_not_exists() {
        let mut pathbuf = get_path();
        pathbuf.push("txt_1.txt");
        let file_path = pathbuf.as_path();
        let search_word = String::from("sun_that_not_exists");
        let result = find_in_file(file_path, &search_word);
        assert!(result.is_some());
    }

    #[test]
    fn test_find_in_dir() {
        let binding = get_path();
        let dir_path = binding.as_path();
        let search_word = String::from("sun");
        let file_ext = Some("txt".to_string());
        let expected = Some(vec![
            vec![OutResult {
                file_name: String::from("some/path/txt_1.txt"),
                word: search_word.to_string(),
                line: 0,
            }],
            vec![OutResult {
                file_name: String::from("some/path/txt_2.txt"),
                word: search_word.to_string(),
                line: 0,
            }],
        ]);
        let result = find_in_dir(dir_path, &search_word, &file_ext);
        assert_eq!(result.is_some(), expected.is_some());
        assert_eq!(result.unwrap().len(), expected.unwrap().len());
    }

    #[test]
    fn test_find_in_dir_path_not_exists() {
        let mut binding = get_path();
        binding.push("dir_that_not_exists");
        let dir_path = binding.as_path();
        let search_word = String::from("sun");
        let file_ext = Some("txt".to_string());
        let result = find_in_dir(dir_path, &search_word, &file_ext);
        assert!(result.is_none());
    }    

}
