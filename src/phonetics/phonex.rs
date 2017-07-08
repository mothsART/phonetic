extern crate regex;

use self::regex::Regex;


static VOWELS: &'static [char] = &['A','E','I','O','U','Y','1','2','3','4'];

fn replace_except_vowel_after(s_in: &str, search_substring: &str, replace_substring: &str) -> String {
    /*
    // if the 2 substrings are similar
    if search_substring == replace_substring {
        return s_in.to_owned();
    }
    // if the replace substring contain the search substring
    if replace_substring.contains(&search_substring) {
        return s_in.to_owned();
    }
    */
    let len_search_substring = search_substring.len();
    let pos = s_in.find(search_substring);
    match pos {
        Some(p) => {
            let next_char = s_in.chars().skip(p + len_search_substring).next();
            match next_char {
                Some(n) => {
                    if VOWELS.contains(&n) {
                        return s_in.to_owned();
                    }
                    else {
                        return s_in.replace(search_substring, replace_substring);
                    }
                }
                None => {
                    return s_in.replace(search_substring, replace_substring);
                }
            }
        }
        None => {
            return s_in.to_owned();
        }
    }
}

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

fn del_duplicates (value: &str) -> String {
    let mut new_value = String::new();
    let mut last_char = ' ';
    for c in value.chars() {
        if c != last_char {
            new_value.push(c);
        }
        last_char = c;
    }
    return new_value;
}

/// Try phonex
///
pub fn phonex(value: &str) -> String {
    let mut value:String = value.trim().to_uppercase();
    if value.len() == 0 {
        return String::new();
    }
    // step 1 : replace Y by I
    value = value.replace("Y", "I");

    // step 2 : delete H if not préfixed by C, S OR P
    let mut re = Regex::new(r"^H").unwrap();
    value = re.replace_all(&*value, "");
    re = Regex::new(r"(?P<c>[^CSP])H").unwrap();
    value = re.replace_all(&*value, "$c");
    value = value.replace( "ESH", "ES");
    value = value.replace( "NSH", "NS");
    value = value.replace( "SH", "CH");

    // step 3 : replace PH by F
    value = value.replace("PH", "F");

    // step 4 => replace g by k in this forth forms
    value = value.replace("GAN", "KAN");
    value = value.replace("GAM", "KAM");
    value = value.replace("GAIN", "KAIN");
    value = value.replace("GAIM", "KAIM");

    // step 5 :
    let patterns = [
        ["AINA", "EINA", "AIMA", "EIMA"],
        ["AINU", "EINU", "AIMU", "EIMU"],
        ["AINE", "EINE", "AIME", "EIME"],
        ["AINI", "EINI", "AIMI", "EIMI"],
        ["AINO", "EINO", "AIMO", "EIMO"]
    ];
    for group in patterns.iter() {
        for pattern in group.iter() {
            value = value.replace(pattern, &*format!("YN{}", pattern.chars().nth(3).unwrap()));
        }
    }

    // step 6
    value = value.replace("EAU", "O");
    value = value.replace("OUA", "2");
    value = value.replace("EIN", "4");
    value = value.replace("AIN", "4");
    value = value.replace("EIM", "4");
    value = value.replace("AIM", "4");

    // step 7 : replace "é" sound
    value = value.replace("É", "Y");
    value = value.replace("È", "Y");
    value = value.replace("Ê", "Y");
    value = value.replace("AI", "Y");
    value = value.replace("EI", "Y");
    value = value.replace("ER", "YR");
    value = value.replace("ESS", "YSS");
    value = value.replace("ET", "YT");

    // step 8 : replace "an" and "in" excepted if there's a vowel after this pattern
    value = replace_except_vowel_after(&*value, "AN", "1");
    value = replace_except_vowel_after(&*value, "AM", "1");
    value = replace_except_vowel_after(&*value, "EN", "1");
    value = replace_except_vowel_after(&*value, "EM", "1");
    value = replace_except_vowel_after(&*value, "IN", "4");

    // step 9 : replace "s" by "z" only if the "s" is preced or followed by a vowel
    value = replace_if_vowel_before_and_after(&*value, "S", "Z");

    // step 10
    value = value.replace("OE", "E");
    value = value.replace("EU", "E");
    value = value.replace("AU", "O");
    value = value.replace("OI", "2");
    value = value.replace("OY", "2");
    value = value.replace("OU", "3");

    // step 11
    value = value.replace("CH", "5");
    value = value.replace("SCH", "5");
    value = value.replace("SH", "5");
    value = value.replace("SS", "S");
    value = value.replace("SC", "S");

    //step 12 : replace "c" by "s" if followed by "e" or "i"
    value = value.replace("CE", "SE");
    value = value.replace("CI", "SI");

    // step 13
    value = value.replace("C", "K");
    value = value.replace("Q", "K");
    value = value.replace("QU", "K");
    value = value.replace("GU", "K");
    value = value.replace("GA", "KA");
    value = value.replace("GO", "KO");
    value = value.replace("GY", "KY");

    // step 14
    value = value.replace("A", "O");
    value = value.replace("D", "T");
    value = value.replace("P", "T");
    value = value.replace("J", "G");
    value = value.replace("B", "F");
    value = value.replace("V", "F");
    value = value.replace("M", "N");

    // step 15
    value = del_duplicates(&*value);

    // step 16
    let v = vec!('T', 'X', 'S', 'Z');
    for t in v {
        if value.ends_with(t)
        {
            value.pop();
            break;
        }
    }
    return value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phonex_1() {
        assert_eq!("", phonex("  "));
    }

    #[test]
    fn phonex_2() {
        assert_eq!("SINON", phonex("symon"));
    }

    #[test]
    fn phonex_3() {
        assert_eq!("IF3", phonex("hibou"));
        assert_eq!("5O", phonex("chat"));
        assert_eq!("NO", phonex("naho"));
        assert_eq!("5OLON", phonex("shalom"));
    }

    #[test]
    fn phonex_4() {
        assert_eq!("FROZE", phonex("phrase"));
    }

    #[test]
    fn phonex_5() {
        assert_eq!("K1", phonex("gan"));
    }

    #[test]
    fn phonex_6() {
        assert_eq!("KYNE", phonex("gaine"));
        assert_eq!("KONYTE", phonex("gamette"));
        assert_eq!("K1FO", phonex("gambas"));
        assert_eq!("K4", phonex("gaim"));
    }

    #[test]
    fn phonex_7() {
        assert_eq!("YT2LE", phonex("étoile"));
        assert_eq!("TORYSE", phonex("paresse"));
    }

    #[test]
    fn phonex_8() {
        assert_eq!("FL1K", phonex("blanc"));
        assert_eq!("TONE", phonex("pane"));
        assert_eq!("1T1TEN1", phonex("entendement"));
        assert_eq!("51TIGNON", phonex("champignon"));
        assert_eq!("R1T1TE", phonex("rempante"));
        assert_eq!("KOKU4", phonex("coquin"));
    }

    #[test]
    fn phonex_9() {
        assert_eq!("KOZE", phonex("case"));
        assert_eq!("KOZIYR", phonex("casier"));
        assert_eq!("5YZE", phonex("chaise"));
        assert_eq!("FIZION", phonex("vision"));
        assert_eq!("K3ZU", phonex("cousu"));
        assert_eq!("KOZY", phonex("causé"));
    }

    #[test]
    fn phonex_10() {
        assert_eq!("IGLO", phonex("igloo"));
        assert_eq!("FILYTE", phonex("fillette"));
    }

    #[test]
    fn phonex_11() {
        assert_eq!("S3RI", phonex("sourit"));
        assert_eq!("OLOR", phonex("alors"));
        assert_eq!("OLE", phonex("allez"));
        assert_eq!("TRI", phonex("prix"));
    }
}
