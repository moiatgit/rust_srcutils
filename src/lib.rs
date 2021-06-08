/*
 * This function gets a java program as a String and returns the headers in that code.
 *
 * It recognizes as headers:
 *
 * - multiple single-line comments ``//`` with possibly blank
 *   inter-lines, that appear as first non blank lines (except ``package``)
 *   or just after lines considered as headers too.
 *
 * - multiple lines in a multi-line comment ``/* … */`` that appear as first
 *   non blank lines (except ``package``)
 *   or just after lines considered as headers too.
 *
 * - any combination of the previous two kinds of headers appearing
 *   continuous, until the first non-blank non-package line.
 */

 pub fn extract_java_headers(contents: &String) -> String {
     enum State {
         Init,
         Package,
         OpenBar,
         NoPackage,
         Line,
         Multiline,
         EndMultiline,
     }
     let mut state = State::Init;
     let mut package_line_len: usize = 0;
     let mut header_str = String::new();
     let package_str = String::new();
     for ch in contents.chars() {
         match state{
             State::Init => {
                 match ch {
                     ' ' | '\t' | '\n' => continue,
                     'p' => {
                         state = State::Package;
                         package_line_len += 1;
                     },
                     '/' => state = State::OpenBar,
                     _ => break,
                 };
             },

             State::Package => {
                 if package_line_len >= package_str.len() {
                     if ch == ';' {
                         state = State::NoPackage;
                     }
                 } else if ch != package_str.chars().nth(package_line_len).unwrap() {
                     break;
                 } else {
                     package_line_len += 1;
                 }
                 continue;
             },

             State::NoPackage => {
                 match ch {
                     ' ' | '\t' | '\n' => continue,
                     '/' => state = State::OpenBar,
                     _ => break
                 };
             },

             State::OpenBar => {
                 match ch {
                     '/' => state = State::Line,
                     '*' => state = State::Multiline,
                     _ => break
                 }
             },

             State::Multiline => {
                 match ch {
                     '*' => state = State::EndMultiline,
                     _ => {
                         header_str.push(ch);
                     }
                 }
             },

             State::Line => {
                 header_str.push(ch);
                 if ch == '\n' {
                     state = State::Init;
                 }
             },

             State::EndMultiline => {
                 if ch == '/' {
                     state = State::NoPackage;
                 } else {
                     header_str.push('*');
                     if ch != '*' {
                         header_str.push(ch);
                         state = State::Multiline;
                     }
                 }
             },
         }
     }
     header_str
 }


#[cfg(test)]
mod tests_extract_java_headers {
    use super::extract_java_headers;

    #[test]
    fn simple_multiline_header() {
        let source = String::from("/* some contents */\nclass HolaMon {}");
        let result = extract_java_headers(&source);
        let expected = String::from(" some contents ");
        assert_eq!(expected, result);
    }

    #[test]
    fn simple_line_header() {
        let source = String::from("// some contents\nclass HolaMon {}");
        let result = extract_java_headers(&source);
        let expected = String::from(" some contents\n");
        assert_eq!(expected, result);
    }

    #[test]
    fn two_line_header() {
        let source = String::from("// some contents\n// and no more\nclass HolaMon {}");
        let result = extract_java_headers(&source);
        let expected = String::from(" some contents\n and no more\n");
        assert_eq!(expected, result);
    }

    #[test]
    fn mixed_header() {
        let source = String::from("/* some contents */// and no more\nclass HolaMon {}");
        let result = extract_java_headers(&source);
        let expected = String::from(" some contents  and no more\n");
        assert_eq!(expected, result);
    }

    #[test]
    fn weird_header() {
        let source = String::from("/** some contents * * * *\n/// and no more\n*/class HolaMon {}");
        let result = extract_java_headers(&source);
        let expected = String::from("* some contents * * * *\n/// and no more\n");
        assert_eq!(expected, result);
    }

    #[test]
    fn no_header() {
        let source = String::from("class HolaMon {}");
        let result = extract_java_headers(&source);
        let expected = String::from("");
        assert_eq!(expected, result);
    }

    #[test]
    fn empty_single_line_header() {
        let source = String::from("//\nclass HolaMon {}");
        let result = extract_java_headers(&source);
        let expected = String::from("\n");
        assert_eq!(expected, result);
    }

    #[test]
    fn empty_multiline_header() {
        let source = String::from("/**/\nclass HolaMon {}");
        let result = extract_java_headers(&source);
        let expected = String::from("");
        assert_eq!(expected, result);
    }

    #[test]
    fn package_header() {
        let source = String::from("package java.util.Molongui;\n/* some contents */\nclass HolaMon {}");
        let result = extract_java_headers(&source);
        let expected = String::from(" some contents ");
        assert_eq!(expected, result);
    }

    #[test]
    fn package_same_line_header() {
        let source = String::from("package java.util.Molongui;/* some contents */\nclass HolaMon {}");
        let result = extract_java_headers(&source);
        let expected = String::from(" some contents ");
        assert_eq!(expected, result);
    }

    #[test]
    fn package_same_singleline_header() {
        let source = String::from("package java.util.Molongui;    // some contents\nclass HolaMon {}");
        let result = extract_java_headers(&source);
        let expected = String::from(" some contents\n");
        assert_eq!(expected, result);
    }

    #[test]
    fn whitespace_surrounding_header() {
        let source = String::from("   \n\n\t\t/* some*/\n   \t\t   \n// contents\nclass HolaMon {}");
        let result = extract_java_headers(&source);
        let expected = String::from(" some contents\n");
        assert_eq!(expected, result);
    }
}
