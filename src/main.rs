use exchange_calculator::{help, ExchangeRate};
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        let _ = match &command[..] {
            "cal" => ExchangeRate::cal(&args[2], args[3].parse::<u32>().unwrap(), &args[4]).await,
            "cur" => ExchangeRate::cur(&args[2]).await,
            "code" => ExchangeRate::currency_code().await,
            "help" | "--help" | "-h" | _ => help(),
        };
    } else {
        let _ = ExchangeRate::cur("USD").await;
    }
}
