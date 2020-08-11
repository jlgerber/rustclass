#[derive(Debug, PartialEq, Eq)]
pub enum LevelSpec {
    Show(String),
    Sequence(String, String),
    Shot(String, String, String),
}

impl LevelSpec {
    pub fn from_show(name: String) -> Self {
        LevelSpec::Show(name)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_show() {
        let ls = LevelSpec::from_show("dev01".to_string());
        let expected = LevelSpec::Show("dev01".to_string());
        assert_eq!(ls, expected);
    }
}
