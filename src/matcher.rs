pub enum MatchError {
    InvalidPattern(String),
}

pub enum PatternType {
    LiteralCharacter(String),
    Digit,
    Word,
    CharacterGroup(String),
}

pub trait Matcher {
    fn matches(&self, input: &str) -> bool;
}

impl Matcher for PatternType {
    fn matches(&self, input: &str) -> bool {
        match self {
            PatternType::LiteralCharacter(p) => input.contains(p),
            PatternType::Digit => input.chars().any(|c| c.is_ascii_digit()),
            PatternType::Word => input.chars().any(|c| c.is_alphanumeric() || c == '_'),
            PatternType::CharacterGroup(p) => {
                let begin_char = match p.chars().next() {
                    Some(c) => c,
                    None => return false,
                };

                match begin_char {
                    '^' => {
                        let exclude_chars = &p[1..];
                        !exclude_chars.chars().all(|c| exclude_chars.contains(c))
                    }
                    _ => input.chars().any(|c| p.contains(c)),
                }
            }
        }
    }
}

impl PatternType {
    pub fn new(pattern: &str) -> Result<PatternType, MatchError> {
        match pattern {
            p if p.len() == 1 => Ok(PatternType::LiteralCharacter(p.to_string())),
            "\\d" => Ok(PatternType::Digit),
            "\\w" => Ok(PatternType::Word),
            p if p.starts_with('[') && p.ends_with(']') => {
                Ok(PatternType::CharacterGroup(p[1..(p.len() - 1)].to_string()))
            }
            _ => Err(MatchError::InvalidPattern(pattern.to_string())),
        }
    }
}
