use nom::{
    bytes::complete::is_not, character::complete::char, error::ErrorKind, sequence::delimited,
    IResult,
};

fn parens(input: &str) -> IResult<&str, &str> {
    delimited(char('('), is_not(")"), char(')'))(input)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///let result = parens("( this is a test )")?;
    let result = char::<_, (&str, ErrorKind)>('(')("( foo")?;
    println!("{:?}", result);
    let result = char::<_, (&str, ErrorKind)>('(')("a(this is good)");
    println!("{:?}", result);

    Ok(())
}
