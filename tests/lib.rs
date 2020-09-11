extern crate catenc;

#[cfg(test)]
mod tests {
    use catenc::lib::{process_lines, process_lines_tokenized};

    #[test]
    fn test_process_lines_text() {
        let mut result = Vec::new();

        let dict = {
            let lines = "A B\nB C\nC B\nC A\nB C".lines();
            let print_fn = |l| { result.push(l); };
            process_lines(lines, false, print_fn)
        };

        assert_eq!(5, result.len());
        assert_eq!("0", result[0]);
        assert_eq!("1", result[1]);
        assert_eq!("2", result[2]);
        assert_eq!("3", result[3]);
        assert_eq!("1", result[4]);

        assert_eq!(4, dict.keys().len());
        assert_eq!("0", dict["A B"]);
        assert_eq!("1", dict["B C"]);
        assert_eq!("2", dict["C B"]);
        assert_eq!("3", dict["C A"]);
    }

    #[test]
    fn test_process_lines_b64() {
        let mut result = Vec::new();

        let dict = {
            let lines = "A B\nB C\nC B\nC A\nB C".lines();
            let print_fn = |l| { result.push(l); };
            process_lines(lines, true, print_fn)
        };

        assert_eq!(5, result.len());
        assert_eq!("AA==", result[0]);
        assert_eq!("AQ==", result[1]);
        assert_eq!("Ag==", result[2]);
        assert_eq!("Aw==", result[3]);
        assert_eq!("AQ==", result[4]);

        assert_eq!(4, dict.keys().len());
        assert_eq!("AA==", dict["A B"]);
        assert_eq!("AQ==", dict["B C"]);
        assert_eq!("Ag==", dict["C B"]);
        assert_eq!("Aw==", dict["C A"]);
    }

    #[test]
    fn test_process_lines_key() {
        let mut result = Vec::new();

        let dicts = {
            let lines = "A B\nB C\nC B\nC A\nB C".lines();
            let print_fn = |l| { result.push(l); };
            process_lines_tokenized(lines, false, " ", vec!(2), print_fn)
        };

        assert_eq!(5, result.len());
        assert_eq!("A 0", result[0]);
        assert_eq!("B 1", result[1]);
        assert_eq!("C 0", result[2]);
        assert_eq!("C 2", result[3]);
        assert_eq!("B 1", result[1]);

        assert_eq!(2, dicts.len());

        let first_column_map = &dicts[0];
        assert!(first_column_map.is_empty());

        let second_column_map = &dicts[1];
        assert_eq!(3, second_column_map.keys().len());
        assert_eq!("0", second_column_map["B"]);
        assert_eq!("1", second_column_map["C"]);
        assert_eq!("2", second_column_map["A"]);
    }
}