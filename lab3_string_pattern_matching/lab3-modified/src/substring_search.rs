use std::io::{self, Write};

use super::ui::{clear_screen, print_page_title, read_choice, wait_for_enter};

pub fn find_kmp(haystack: &str, needle: &str) -> Option<usize> {
    let text: Vec<_> = haystack.chars().collect();
    let pattern: Vec<_> = needle.chars().collect();

    // Note:
    // `i` and `j` are used to denote the next character to be matched.

    let mut highest_pos_with_known_lps: usize = 0;
    let (mut i, mut j) = (0, 0);
    let mut lps: Vec<usize> = vec![0; pattern.len()];

    // Longest Proper Prefix which is also a suffix

    while i < text.len() && j < pattern.len() {
        if j > 0 && j > highest_pos_with_known_lps {
            let mut k = lps[j - 1];

            while k > 0 && pattern[j] != pattern[k] {
                k = lps[k - 1];
            }

            // case 1: k > 0 (we've found some really available bound.)
            // case 2: k == 0 (we've not found any reasonable bound yet.)
            if pattern[j] == pattern[k] {
                k += 1;
            }

            lps[j] = k;
            highest_pos_with_known_lps = j;
        }
        if text[i] == pattern[j] {
            i += 1;
            j += 1;
        } else if j > 0 {
            // We have the chance to just backtrack.
            j = lps[j - 1];
        } else {
            i += 1;
        }
    }

    (j == pattern.len()).then_some(i - j)
}

pub fn run() {
    loop {
        clear_screen();
        print_page_title("The KMP pattern matching algorithm");

        println!("  1. Perform a KMP pattern matching");
        println!("  0. Return to the main menu");
        println!();

        match read_choice() {
            1 => {
                let mut source = String::new();
                let mut pattern = String::new();

                println!("Please provide the source string, and the pattern string to search for.");

                // 1. The source string
                print!("The source string: ");
                io::stdout().flush().expect("Failed to flush the output!");
                io::stdin()
                    .read_line(&mut source)
                    .expect("Failed to read in the source string!");

                // 2. The pattern string
                print!("The pattern string: ");
                io::stdout().flush().expect("Failed to flush the output!");
                io::stdin()
                    .read_line(&mut pattern)
                    .expect("Failed to read in the pattern string!");

                println!();
                match find_kmp(&source.trim_end_matches(['\n', 'r']), &pattern.trim_end_matches(['\n', 'r'])) {
                    Some(position) => println!(
                        "The first occurence of the string is found at {}.",
                        position
                    ),
                    None => {
                        println!("We haven't found any instances of the pattern string!");
                    }
                }
            }
            0 => return,
            _ => println!("Invalid choice — please enter 0 or 1."),
        }

        wait_for_enter();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_matching() {
        let cases = [
            ("Hello, world", "world", Some(7)),
            ("Hello, world", "World", None),
            ("Hello, world", "worldsomeothersuffix", None),
            ("world, world", "world", Some(0)),
            ("abcabc", "abc", Some(0)),
            ("xxabc", "abc", Some(2)),
            ("abcxx", "abc", Some(0)),
            ("abc", "a", Some(0)),
            ("abc", "b", Some(1)),
            ("abc", "c", Some(2)),
            ("abc", "ab", Some(0)),
            ("abc", "bc", Some(1)),
            ("abc", "ac", None),
            ("abc", "d", None),
        ];

        for (haystack, needle, expected) in cases {
            assert_eq!(
                find_kmp(haystack, needle),
                expected,
                "haystack={haystack:?}, needle={needle:?}"
            );
        }
    }

    #[test]
    fn boundary_cases() {
        let cases = [
            ("", "", Some(0)),
            ("", "a", None),
            ("a", "", Some(0)),
            ("a", "a", Some(0)),
            ("a", "b", None),
            ("abc", "abc", Some(0)),
            ("abc", "abcd", None),
            ("aaaaa", "aaa", Some(0)),
            ("aaaaa", "aaaaaa", None),
            ("aaaaa", "aaab", None),
        ];

        for (haystack, needle, expected) in cases {
            assert_eq!(
                find_kmp(haystack, needle),
                expected,
                "haystack={haystack:?}, needle={needle:?}"
            );
        }
    }

    #[test]
    fn backtracking_and_overlapping() {
        let cases = [
            ("aaab", "aab", Some(1)),
            ("aaaab", "aaab", Some(1)),
            ("aaaaab", "aaab", Some(2)),
            ("aaaaa", "aaab", None),
            ("ababac", "abac", Some(2)),
            ("ababababac", "ababac", Some(4)),
            ("abababac", "ababac", Some(2)),
            ("abababab", "ababac", None),
            ("abcabcabd", "abcabd", Some(3)),
            ("abcabcabcabd", "abcabd", Some(6)),
            ("ababcabcabababd", "ababd", Some(10)),
            ("abcxabcdabxabcdabcdabcy", "abcdabcy", Some(15)),
            ("aaaaaaaaab", "aaaaab", Some(4)),
            ("aaaaaaaac", "aaaaab", None),
            ("abababa", "ababa", Some(0)),
            ("abababa", "baba", Some(1)),
            ("ababababx", "abababx", Some(2)),
            ("aabaaab", "aaab", Some(3)),
            ("abcabcabc", "cab", Some(2)),
            ("abcabcabc", "cac", None),
            ("ababaaaba", "abaa", Some(2)),
        ];

        for (haystack, needle, expected) in cases {
            assert_eq!(
                find_kmp(haystack, needle),
                expected,
                "haystack={haystack:?}, needle={needle:?}"
            );
        }
    }

    #[test]
    fn false_positive_regressions() {
        let cases = [
            ("abbc", "abc", None),
            ("abc", "ac", None),
            ("aaaaa", "aaab", None),
            ("abababab", "ababac", None),
            ("abcabcabc", "cac", None),
            ("aa", "a\n", None),
        ];

        for (haystack, needle, expected) in cases {
            assert_eq!(
                find_kmp(haystack, needle),
                expected,
                "haystack={haystack:?}, needle={needle:?}"
            );
        }
    }

    #[test]
    fn unicode() {
        let cases = [
            ("你好，世界", "世界", Some(3)),
            ("你好你好", "你好", Some(0)),
            ("汉字汉字", "字汉", Some(1)),
            ("🙂🙃🙂🙂", "🙂🙂", Some(2)),
            ("🙂🙂🙂", "🙂🙂", Some(0)),
            ("🍎🍌🍎🍇", "🍎🍇", Some(2)),
            ("ééx", "éx", Some(1)),
            ("aébc", "bc", Some(2)),
            ("e\u{301}x", "\u{301}x", Some(1)),
            ("é", "e\u{301}", None),
            ("你好, hello", "hello", Some(4)),
        ];

        for (haystack, needle, expected) in cases {
            assert_eq!(
                find_kmp(haystack, needle),
                expected,
                "haystack={haystack:?}, needle={needle:?}"
            );
        }
    }

    #[test]
    fn whitespace_and_special_characters() {
        let cases = [
            ("a\nb", "\nb", Some(1)),
            ("a\0b", "\0b", Some(1)),
            ("a b", " b", Some(1)),
            (" a ", "a ", Some(1)),
            ("line\r\nend", "\r\n", Some(4)),
            ("a\tb", "\tb", Some(1)),
            ("aa", "a\n", None),
        ];

        for (haystack, needle, expected) in cases {
            assert_eq!(
                find_kmp(haystack, needle),
                expected,
                "haystack={haystack:?}, needle={needle:?}"
            );
        }
    }

    #[test]
    fn long_repetitive_input() {
        let haystack = format!("{}b", "a".repeat(2048));
        let needle = format!("{}b", "a".repeat(1024));

        assert_eq!(find_kmp(&haystack, &needle), Some(1024));

        let haystack_without_match = "a".repeat(2048);

        assert_eq!(
            find_kmp(&haystack_without_match, &needle),
            None
        );
    }
}