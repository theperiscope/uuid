use std::env;
use std::process;
use uuid::Uuid;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("{}", err);
    process::exit(1);
  });

  for _ in 0..config.count {
    let value = Uuid::new_v4();

    // https://stackoverflow.com/questions/25383488/how-to-match-a-string-against-string-literals-in-rust
    match config.outputformat.as_ref() {
      "-b" => println!("{:?}", value.as_bytes()),
      "-h" => println!("{}", value.to_hyphenated()),
      "-H" => println!("{}", value.to_hyphenated().to_string().to_uppercase()),
      "-s" => println!("{}", value.to_simple()),
      "-S" => println!("{}", value.to_simple().to_string().to_uppercase()),
      "-u" => println!("{}", value.to_urn()),
      "-U" => println!("{}", value.to_urn().to_string().to_uppercase()),
      "-a" | _ => {
        println!("{:?}", value.as_bytes());
        println!("{}", value.to_hyphenated());
        println!("{}", value.to_hyphenated().to_string().to_uppercase());
        println!("{}", value.to_simple());
        println!("{}", value.to_simple().to_string().to_uppercase());
        println!("{}", value.to_urn());
        println!("{}", value.to_urn().to_string().to_uppercase());
      }
    }
  }
}

struct Config {
  count: u32,
  outputformat: String,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &'static str> {
    // just .exe
    if args.len() == 1 {
      return Ok(Config {
        count: 1,
        outputformat: "-a".to_string(),
      });

    // one argument - can be either the count or the output format
    } else if args.len() == 2 {
      // one argument: output format
      if args[1].starts_with('-') {
        return Ok(Config {
          count: 1,
          outputformat: args[1].clone().to_string(),
        });
      }

      // one argument: count
      let mut count = args[1].parse::<u32>().unwrap();
      if count <= 0 {
        count = 1
      }

      return Ok(Config {
        count: count,
        outputformat: "-a".to_string(),
      });

    // second argument is output format type: binary, hyphenated, simple, bytes
    } else if args.len() == 3 {
      return Ok(Config {
        count: args[1].parse::<u32>().unwrap(),
        outputformat: args[2].clone().to_string(),
      });
    }

    return Err("Invalid arguments.");
  }
}
