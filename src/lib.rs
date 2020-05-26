use regex::Regex;

trait MatcherTrait {
    fn execute(&self, line: &str) -> bool;
}

#[derive(Clone)]
pub struct ExtendedRegexpMatcher {
    pattern: Regex,
}
impl ExtendedRegexpMatcher {
    pub fn new(pattern: String) -> ExtendedRegexpMatcher {
        ExtendedRegexpMatcher {
            pattern: Regex::new(&pattern).unwrap(),
        }
    }
}
impl MatcherTrait for ExtendedRegexpMatcher {
    fn execute(&self, line: &str) -> bool {
        self.pattern.is_match(line)
    }
}

#[derive(Clone)]
pub struct FixedStringMatcher {
    pattern: String,
}
impl FixedStringMatcher {
    pub fn new(pattern: String) -> FixedStringMatcher {
        FixedStringMatcher { pattern: pattern }
    }
}
impl MatcherTrait for FixedStringMatcher {
    fn execute(&self, line: &str) -> bool {
        line.contains(&self.pattern)
    }
}

#[derive(Clone)]
pub enum Matcher {
    ExtendedRegexp(ExtendedRegexpMatcher),
    FixedString(FixedStringMatcher),
}
impl Matcher {
    pub fn new(pattern: String, is_fixed_strings_mode: bool) -> Matcher {
        if is_fixed_strings_mode {
            Matcher::FixedString(FixedStringMatcher::new(pattern.to_string()))
        } else {
            Matcher::ExtendedRegexp(ExtendedRegexpMatcher::new(pattern.to_string()))
        }
    }
    pub fn execute(&self, line: &str) -> bool {
        match self {
            Matcher::FixedString(m) => m.execute(line),
            Matcher::ExtendedRegexp(m) => m.execute(line),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_extended_regexp_matcher() {
        let matcher = Matcher::new("c".to_string(), false);
        assert_eq!(true, matcher.execute("abcdefg"));
        let matcher = Matcher::new("fg".to_string(), false);
        assert_eq!(true, matcher.execute("abcedefg"));
        let matcher = Matcher::new("Z".to_string(), false);
        assert_eq!(false, matcher.execute("abcdefg"));
        let matcher = Matcher::new("a.c".to_string(), false);
        assert_eq!(true, matcher.execute("abcdefg"));
        let matcher = Matcher::new("a+.b+".to_string(), false);
        assert_eq!(true, matcher.execute("aaa bbb"));
        let matcher = Matcher::new("[aA][bB][cC]".to_string(), false);
        assert_eq!(true, matcher.execute("aBc"));
        assert_eq!(true, matcher.execute("Abc"));
    }

    #[test]
    fn text_fixed_string_matcher() {
        let matcher = Matcher::new("c".to_string(), true);
        assert_eq!(true, matcher.execute("abcdefg"));
        assert_eq!(true, matcher.execute("cccc"));
        let matcher = Matcher::new("fg".to_string(), true);
        assert_eq!(true, matcher.execute("abcdefg"));
        let matcher = Matcher::new("Z".to_string(), true);
        assert_eq!(false, matcher.execute("abcdefg"));
        let matcher = Matcher::new("a.c".to_string(), true);
        assert_eq!(false, matcher.execute("abcdefg"));
        let matcher = Matcher::new("a+.b+".to_string(), true);
        assert_eq!(false, matcher.execute("aaa bbb"));
        let matcher = Matcher::new("[aA][bB][cC]".to_string(), true);
        assert_eq!(false, matcher.execute("aBc"));
    }
}
