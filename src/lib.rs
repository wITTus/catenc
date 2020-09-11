pub mod lib {
    use std::borrow::Borrow;
    use std::cmp::max;
    use std::collections::HashMap;
    use std::fs;

    pub fn process_lines<F: FnMut(String), I>(lines: I, mode_base64: bool, mut print_fn: F)
                                              -> HashMap<String, String>
        where I: Iterator,
              I::Item: Borrow<str>,
    {
        let mut dict = HashMap::new();
        let mut counter = 0;

        lines.for_each(|line| {
            let l = line.borrow();
            let e = process_token(l.to_string(), &mut dict, &mut counter, mode_base64);
            print_fn(e);
        });

        return dict;
    }

    pub fn process_lines_tokenized<F: FnMut(String), I>(lines: I, mode_base64: bool, separator: &str, indices: Vec<usize>, mut print_fn: F)
                                                        -> Vec<HashMap<String, String>>
        where I: Iterator,
              I::Item: Borrow<str>,
    {
        let n_indices = *indices.iter().max().unwrap_or(&1);

        let mut dicts: Vec<HashMap<String, String>> = Vec::with_capacity(n_indices);
        dicts.resize(n_indices, HashMap::new());

        let mut counters: Vec<usize> = Vec::with_capacity(n_indices);
        counters.resize(n_indices, 0);

        lines
            .map(|l| l.borrow().split(separator)
                .enumerate()
                .map(|(i, token)|
                    if indices.contains(&(i + 1)) {
                        process_token(token.to_string(), &mut dicts[i], &mut counters[i], mode_base64)
                    } else {
                        token.to_string()
                    }
                )
                .collect::<Vec<String>>()
                .join(separator)
            )
            .for_each(|t| print_fn(t));

        return dicts;
    }

    fn process_token(line: String, dict: &mut HashMap<String, String>, counter: &mut usize, mode_base64: bool) -> String {
        let e = dict.entry(line).or_insert_with(|| {
            let s = if mode_base64 {
                transform_b64(*counter)
            } else {
                transform_identity(*counter)
            };
            *counter += 1;
            s
        });
        e.to_string()
    }

    fn transform_identity(counter: usize) -> String {
        counter.to_string()
    }

    fn transform_b64(counter: usize) -> String {
        let raw_bytes: [u8; 8] = unsafe { std::mem::transmute(counter) };
        let count = max(1, raw_bytes.iter().filter(|&b| *b > 0).count());
        base64::encode(&raw_bytes[0..count])
    }

    pub fn invert_hashmap<T>(m: &HashMap<T, T>) -> HashMap<&T, &T>
        where T: std::cmp::Eq,
              T: std::hash::Hash
    {
        let mut r = HashMap::new();
        for (k, v) in m {
            r.insert(v, k);
        }
        r
    }

    pub fn write_dictionary(filepath: &str, dicts: Vec<HashMap<String, String>>) {
        let reversed_dict: Vec<HashMap<&String, &String>> = dicts
            .iter()
            .map(|m| invert_hashmap(&m))
            .collect();

        let data = serde_json::to_string(&reversed_dict).expect("Couldn't generate JSON dictionary");
        fs::write(filepath, data).expect("Unable to write file");
    }
}

