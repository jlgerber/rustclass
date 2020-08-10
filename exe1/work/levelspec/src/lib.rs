//! LevelSpec - creates a levelspec duh?
pub mod error;
pub use error::LevelSpecError;
use std::env;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LevelSpec {
    Show {
        show: String,
    },
    Sequence {
        show: String,
        sequence: String,
    },
    Shot {
        show: String,
        sequence: String,
        shot: String,
    },
}

impl LevelSpec {
    /// new can be called with Option of any type which implements Into<String>..
    /// new(Some("bla")...)
    ///
    /// # Example
    /// ```
    /// fn main() {
    ///     use levelspec::LevelSpec;
    ///     let show = LevelSpec::new(Some("show"), None, None);
    /// }
    /// ```
    pub fn new<I>(
        show: Option<I>,
        sequence: Option<I>,
        shot: Option<I>,
    ) -> Result<Self, LevelSpecError>
    where
        I: Into<String> + std::fmt::Debug,
    {
        match (show, sequence, shot) {
            (Some(show), None, None) => Ok(LevelSpec::Show { show: show.into() }),
            (Some(show), Some(seq), None) => Ok(LevelSpec::Sequence {
                show: show.into(),
                sequence: seq.into(),
            }),
            (Some(show), Some(seq), Some(shot)) => Ok(LevelSpec::Shot {
                show: show.into(),
                sequence: seq.into(),
                shot: shot.into(),
            }),
            _ => Err(LevelSpecError::InvalidInputError(
                "new called with invalid args ".to_string(),
            )),
        }
    }

    /// construct from environment
    pub fn from_env() -> Result<Self, LevelSpecError> {
        LevelSpec::new(
            env::var("DD_SHOW").ok(),
            env::var("DD_SEQUENCE").ok(),
            env::var("DD_SHOT").ok(),
        )
    }
}

fn chartest(val: &str) -> Result<&str, LevelSpecError> {
    if val.chars().any(|x| !x.is_alphanumeric() && x != '_') {
        Err(LevelSpecError::InvalidCharactersInInput(val.to_string()))
    } else {
        Ok(val)
    }
}
impl FromStr for LevelSpec {
    type Err = LevelSpecError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let vals = input
            .split(".")
            .map(chartest)
            .collect::<Result<Vec<_>, LevelSpecError>>()?;

        match vals.as_slice() {
            [show, seq, shot] => LevelSpec::new(Some(*show), Some(*seq), Some(*shot)),
            [show, seq] => LevelSpec::new(Some(*show), Some(*seq), None),
            [show] => LevelSpec::new(Some(*show), None, None),
            _ => Err(LevelSpecError::InvalidInputError(
                "Invalid input to from_str".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_generate_levelspec_from_new() {
        let ls = LevelSpec::new(Some("dev01"), None, None);
        assert!(ls.is_ok());
        let ls = LevelSpec::new(Some("dev01"), Some("rd"), None);
        assert!(ls.is_ok());
    }

    #[test]
    fn can_construct_from_env() {
        // set up environment
        env::set_var("DD_SHOW", "DEV01");
        env::set_var("DD_SEQUENCE", "RD");
        let ls = LevelSpec::from_env();
        let expect = LevelSpec::Sequence {
            show: "DEV01".to_string(),
            sequence: "RD".to_string(),
        };
        assert!(ls.is_ok());
        assert_eq!(ls.unwrap(), expect);
    }

    #[test]
    fn can_init_from_str() {
        let ls = LevelSpec::from_str("DEV012.RD.9999");
        assert!(ls.is_ok());
        let expect = LevelSpec::Shot {
            show: "DEV012".to_string(),
            sequence: "RD".to_string(),
            shot: "9999".to_string(),
        };
        assert_eq!(ls.unwrap(), expect);
    }
}
