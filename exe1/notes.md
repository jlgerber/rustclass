
Result implements FromIterator, so you can move the Result outside and iterators will take care of the rest (including stopping iteration if an error is found).
```rust
fn chartest(val: &str) -> Result<&str, String> {
    if val.chars().any(|x| ! x.is_alphanumeric() && x!='_') {
        Err(format!("invalid input"))
    } else {
        Ok(val)
    }
}

fn main() -> Result<(),String> {
    
    let foo = "this_foo.is.it".split(".")
                .map(chartest)
                .collect::<Result<Vec<_>, String>>()?;
    println!("{:?}", foo);
    Ok(())
}
```

we can mathc as well
```rust
match foo.as_slice() {
        [show,seq,shot] => println!("{} {} {}", show, seq, shot),
        [show,seq] => println!("{} {}",show, seq ),
        [show] => println!("{}", show),
        _ => println!("error")
    }
```

```rust
// 1.0.20
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
```
```rust
The messages support a shorthand for interpolating fields from the error.

    #[error("{var}")] ⟶ write!("{}", self.var)
    #[error("{0}")] ⟶ write!("{}", self.0)
    #[error("{var:?}")] ⟶ write!("{:?}", self.var)
    #[error("{0:?}")] ⟶ write!("{:?}", self.0)
```

fn idea
```rust

pub fn new(show:Option<I>, seq:Option<I>, shot: Option<I>) -> Result<bla, error> where I: Into<String> {
    match (show, seq,shot) {
         (Some(show), Some(Seq), Some(shot)) => 
         (Some(show), Some(seq), None)
         (Some(show), None, None)
         _ => Err(bla)
    }
}
///
pub fn from_env() -> Result<level, Err> {
    Level::new(env::var("DD_SHOW").ok(), env::var("DD_SEQUENCE").ok(), env::var("DD_SHOT").ok())
}
```

tests 
cargo test -- --test-threads 1