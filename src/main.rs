struct Args {
    thing: String,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut thing = None;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Value(val) if thing.is_none() => {
                thing = Some(val.string()?);
            }
            Long("help") => {
                println!("Usage: shouty [-n|--number=NUM] [--shout] THING");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        thing: thing.ok_or("missing argument THING")?,
    })
}

fn main() -> Result<(), lexopt::Error> {
    let args = parse_args()?;
    println!("{}", args.thing);
    Ok(())
}