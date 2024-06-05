use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Connect { address: String },
    Quit,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let tokens = string.split_whitespace().collect::<Vec<&str>>();
        if !tokens.is_empty() {
            match tokens[0] {
                "connect" => {
                    if tokens.len() != 2 {
                        Err(anyhow::anyhow!("'connect' command only takes one argument"))
                    } else {
                        Ok(Self::Connect {
                            address: tokens[1].to_string(),
                        })
                    }
                }
                "quit" => Ok(Self::Quit),
                _ => Err(anyhow::anyhow!("Unknown command '{}'", tokens[0])),
            }
        } else {
            Err(anyhow::anyhow!("Empty command"))
        }
    }
}
