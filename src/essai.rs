static VOWELS: &'static [char] = &['A','E','I','O','U','Y','1','2','3','4'];

fn replace_if_vowel_before_and_after(s_in: &str, search_substring: &str, replace_substring: &str) -> String {
    let len_search_substring = search_substring.len();
    let pos = s_in.find(search_substring);
    match pos {
        Some(0) => {
            return s_in.to_owned();
        }
        Some(p) => {
            let next_char = s_in.chars().skip(p + len_search_substring).next();
            match next_char {
                Some(n) => {
                    let last_char = s_in.chars().skip(p + len_search_substring - 2).next();
                    if VOWELS.contains(&n) {
                        match last_char {
                            Some(p) => {
                                if VOWELS.contains(&p) {
                                    return s_in.replace(search_substring, replace_substring);
                                }
                                else {
                                    return s_in.to_owned();
                                }
                            }
                            None => {
                                return s_in.to_owned();
                            }
                        }
                    }
                    else {
                        return s_in.to_owned();
                    }
                }
                None => {
                    return s_in.to_owned();
                }
            }
        }
        None => {
            return s_in.to_owned();
        }
    }
}

fn main() {
    println!("{:?}", replace_if_vowel_before_and_after("SIMON", "S", "Z"));
}