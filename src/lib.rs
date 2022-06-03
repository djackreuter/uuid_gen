use std::error::Error;
use uuid::Uuid;

pub struct Config {
    url: String,
    num: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!\n\tUsage: uuid_gen https://site.com?param <num to generate>");
        }
        let url = args[1].clone();
        let num = args[2].clone();

        Ok(Config { url, num })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let url = config.url;
    let num = config.num;

    let n: u32 = num.parse().unwrap();
    if n > 100 {
        Err("Too high! Limit 100")?;
    }

    let mut current = 0;
    while current < n {
        let id = Uuid::new_v4();
        println!("{}={}", url, id);
        current += 1;
    }

    Ok(())
}
