use rache::common_enums::{Request, Response};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use std::io::{self, Write};

#[derive(StructOpt, Debug)]
#[structopt(name = "client")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Read { key: String },
    Write { key: String, value: String },
    Delete { key: String },
}

impl Opt {
    fn from_input(input: &str) -> Result<Self, structopt::clap::Error> {
        let mut args = vec!["client"];
        args.extend(input.split_whitespace());
        Self::from_iter_safe(args)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:6666").await.unwrap();
    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let opt = match Opt::from_input(input.trim()) {
            Ok(opt) => opt,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        let command = match opt.cmd {
            Command::Read { key } => Request::Read { key },
            Command::Write { key, value } => Request::Write { key, value },
            Command::Delete { key } => Request::Delete { key },
        };

        let mut buf = Vec::new();
        command.serialize(&mut Serializer::new(&mut buf)).unwrap();
        stream.write_all(&buf).await.unwrap();
        stream.flush().await.unwrap();

        let mut reader = BufReader::new(&mut stream);
        let mut response_buf = Vec::new();
        reader.read_buf(&mut response_buf).await.unwrap();
        let mut de = Deserializer::new(&response_buf[..]);
        let response: Response = Deserialize::deserialize(&mut de).unwrap();
        println!("Response: {:?}", response);
    }
}
