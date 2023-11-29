// use std::{env, process};
use dotenv::dotenv;
use reqwest::Error as ReqwestError;
use serde_json::{Error, Value};
use std::collections::HashMap;
use std::{env, fmt};

#[derive(Debug)]
pub struct ExchangeRate {
    pub cur_unit: String,
    pub buy_ex_rate: f64,
    pub sell_ex_rate: f64,
    pub cur_name: String,
}

#[derive(Debug)]
struct NotFoundError {
    key: String,
    value: String,
}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "No object found with key '{}' and value '{}'",
            self.key, self.value
        )
    }
}

impl std::error::Error for NotFoundError {}

impl ExchangeRate {
    pub async fn currency_exchange(cur_unit: &str) {
        let an: Result<String, ReqwestError> = get_request().await;
        let str_an: &str = &an.unwrap();
        // println!("cur func str_an ::::{:?}", str_an); // array of json
        let a = get_map_from_json(str_an, "cur_unit", cur_unit); // array of json
        println!("cur func a ::::{:?}", a);
        let value = &a.unwrap().unwrap();
        let res = Self {
            cur_unit: cur_unit.to_string(),
            buy_ex_rate: value["tts"]
                .to_string()
                .replace("\"", "")
                .replace(",", "")
                .parse::<f64>()
                .unwrap(),
            sell_ex_rate: value["ttb"]
                .to_string()
                .replace("\"", "")
                .replace(",", "")
                .parse::<f64>()
                .unwrap(),
            cur_name: value["cur_nm"].to_string().replace("\"", ""),
        };
        println!(
            "통화코드: {}, 송금 보낼 때 환율: {}, 송금 받을 때 환율: {}, 통화 이름: {}",
            res.cur_unit, res.buy_ex_rate, res.sell_ex_rate, res.cur_name
        );
    }
    pub async fn calculate(cur_unit: &str, money: u32, case: &str) {
        let an: Result<String, ReqwestError> = get_request().await;
        let str_an: &str = &an.unwrap();
        let a = get_map_from_json(str_an, "cur_unit", cur_unit);
        let value = &a.unwrap().unwrap();
        let cur_name = value["cur_nm"].to_string().replace("\"", "");
        let buy_ex_rate: f64 = value["tts"]
            .to_string()
            .replace("\"", "")
            .replace(",", "")
            .parse::<f64>()
            .unwrap();
        let sell_ex_rate: f64 = value["ttb"]
            .to_string()
            .replace("\"", "")
            .replace(",", "")
            .parse::<f64>()
            .unwrap();
        if case == "buy" {
            println!(
                "Exchange {} 원 to {} becomes {:?} {:?} ",
                money,
                cur_name,
                (money as f64 / buy_ex_rate * 100.0).round() / 100.0,
                cur_unit,
            );
        } else {
            println!(
                "Exchange {} {} to 원 becomes {:?} 원 ",
                money,
                cur_name,
                (sell_ex_rate * money as f64) as u32,
            );
        }
    }
    pub async fn currency_code() {
        let an: Result<String, ReqwestError> = get_request().await;
        let str_an: &str = &an.unwrap();
        let v: Value = serde_json::from_str(str_an).unwrap();
        v.as_array().unwrap().iter().for_each(|x| {
            println!("Code: {}, Name: {} ", x["cur_unit"], x["cur_nm"]);
        });
    }
}

fn get_map_from_json(json_str: &str, key: &str, value: &str) -> Result<Option<Value>, Error> {
    let v: Value = serde_json::from_str(json_str).unwrap();
    if let Value::Array(array) = v {
        for item in array {
            if let Value::Object(obj) = item {
                if obj.get(key) == Some(&Value::String(value.to_string())) {
                    return Ok(Some(Value::Object(obj)));
                }
            }
        }
    }
    panic!("No object found with key '{}' and value '{}'", key, value);
}

async fn get_request() -> Result<String, ReqwestError> {
    dotenv().ok();
    let env_api_key: String = env::var("AUTHKEY").unwrap();
    let api_key: &str = &env_api_key;
    println!("get_request ;;;;; test path");
    let url = "https://www.koreaexim.go.kr/site/program/financial/exchangeJSON";
    // Set up the query parameters for the request
    let mut params = HashMap::new();

    params.insert("authkey", api_key);
    params.insert("data", "AP01");
    let response = reqwest::Client::new()
        .get(url)
        .query(&params)
        .send()
        .await?;

    let body = response.text().await?;
    Ok(body)
}

const HELP: &str = r"
    Usage: cargo run [COMMAND] [ARGUMENTS]
    Exchange_rate calculator is simple to search exchange rate information and calculate exchange by rust.
    Example: cargo run cur USD 
    Available commands:
        - cal    
            Calculate exchange rate buy or sell 
            args = [cur_unit] [money] [case]
            Example Buy case: cargo run cal USD 1000 buy
            Example Sell case: cargo run cal USD 1000 sell
        cur     
            Get current exchange rate
            args = [cur_unit]
            Example: cargo run cur USD
        help    
            Display help information
    ";

pub fn help() {
    println!("{}", HELP);
}

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use httpmock::prelude::*;
    use serde_json::Value;
    use std::fs;

    use crate::{get_map_from_json, get_request};

    #[tokio::test]
    async fn test_real_api_request() {
        let result = get_request().await;
        assert!(result.is_ok());
    }
    #[test]
    fn test_get_map_from_json() {
        let mock_body: String = fs::read_to_string("fixtures.json").unwrap();
        let json_body: Value = serde_json::from_str(&mock_body).unwrap();
        let target = json_body.as_array().unwrap();
        let result = get_map_from_json(&mock_body, "cur_unit", "USD");
        let res = result.unwrap().unwrap();
        assert_eq!(res, target[0]);
    }
    #[tokio::test]
    async fn test_currency_exchange() {
        use crate::ExchangeRate;
        dotenv().ok();
        let server = MockServer::start();
        let env_api_key: String = std::env::var("AUTHKEY").unwrap();
        let api_key: &str = &env_api_key;
        let mock_body = fs::read_to_string("fixtures.json").unwrap();

        let _mock = server.mock(|when, then| {
            when.method(GET)
                .path("/site/program/financial/exchangeJSON")
                .query_param("authkey", api_key)
                .query_param("data", "AP01");
            then.status(200).body(&mock_body);
        });

        ExchangeRate::currency_exchange("USD").await;
    }
}
