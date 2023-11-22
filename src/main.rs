mod lib;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let arg_money: u32 = args[2].parse::<u32>().unwrap();
        let _ = lib::ExchangeRate::cal(&args[1], arg_money, &args[3]).await;
    } else {
        let _ = lib::ExchangeRate::cur("USD").await;
    }
}
