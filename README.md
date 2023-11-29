# exchange calculator by Standard Won

### cargo build

    cargo build

### env config

- personally api_key is required
- env variable AUTHKEY used
- Obtain an api_key from https://www.koreaexim.go.kr/ir/HPHKIR020M01?apino=2&viewtype=O#tab1 and specify it in the AUTHKEY value.

### running code

- command params

  - cal `cargo run cal USD 1000 buy` - calculate money to exchange currency
  - cur `cargo run cur USD` - search to exchange won to currency code and currency code to won
  - code `cargo run code` - search currency code ex) USD - usa dollar
  - help `cargo run help` - explain command set

  `cargo run Currency_code: String money: u32 buy or sell:String`

```
// Example
cargo run USD 1000 buy
❯ Exchange 10000 dollar to 원 becomes 13005699 원
```
