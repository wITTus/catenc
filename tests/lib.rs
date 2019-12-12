extern crate catenc;

#[cfg(test)]
mod tests {
    use catenc::lib::{process_lines, process_lines_tokenized};

    #[test]
    fn test_process_lines_text() {
        let mut result = Vec::new();
        {
            let lines = "A B\nB C\nC B\nC A\nB C".lines();
            let print_fn = |l| { result.push(l); };
            process_lines(lines, false, print_fn);
        }
        assert_eq!(5, result.len());
        assert_eq!("0", result[0]);
        assert_eq!("1", result[1]);
        assert_eq!("2", result[2]);
        assert_eq!("3", result[3]);
        assert_eq!("1", result[4]);
    }

    #[test]
    fn test_process_lines_b64() {
        let mut result = Vec::new();
        {
            let lines = "A B\nB C\nC B\nC A\nB C".lines();
            let print_fn = |l| { result.push(l); };
            process_lines(lines, true, print_fn);
        }
        assert_eq!(5, result.len());
        assert_eq!("AA==", result[0]);
        assert_eq!("AQ==", result[1]);
        assert_eq!("Ag==", result[2]);
        assert_eq!("Aw==", result[3]);
        assert_eq!("AQ==", result[4]);
    }

    #[test]
    fn test_process_lines_key() {
        let mut result = Vec::new();
        {
            let lines = "A B\nB C\nC B\nC A\nB C".lines();
            let print_fn = |l| { result.push(l); };
            process_lines_tokenized(lines, false, " ", vec!(2), print_fn);
        }
        assert_eq!(5, result.len());
        assert_eq!("A 0", result[0]);
        assert_eq!("B 1", result[1]);
        assert_eq!("C 0", result[2]);
        assert_eq!("C 2", result[3]);
        assert_eq!("B 1", result[1]);
    }
}