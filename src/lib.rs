// use std::{env, process};
use dotenv::dotenv;
use reqwest::Error as ReqwestError;
use serde_json::{Error, Value};
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct ExchangeRate {
    pub cur_unit: String,
    pub buy_ex_rate: f64,
    pub sell_ex_rate: f64,
    pub cur_name: String,
}

impl ExchangeRate {
    pub async fn cur(cur_unit: &str) -> Result<Self, String> {
        let an: Result<String, ReqwestError> = get_request().await;
        let str_an: &str = &an.unwrap();
        let a = get_map_from_json(str_an, "cur_unit", cur_unit);
        let value = &a.unwrap().unwrap();
        println!("{:?}", value);
        Ok(Self {
            cur_unit: cur_unit.to_string(),
            buy_ex_rate: value["ttb"]
                .to_string()
                .replace("\"", "")
                .replace(",", "")
                .parse::<f64>()
                .unwrap(),
            sell_ex_rate: value["tts"]
                .to_string()
                .replace("\"", "")
                .replace(",", "")
                .parse::<f64>()
                .unwrap(),
            cur_name: value["cur_nm"].to_string().replace("\"", ""),
        })
    }
    pub async fn cal(cur_unit: &str, money: u32, case: &str) -> Result<Self, String> {
        let an: Result<String, ReqwestError> = get_request().await;
        let str_an: &str = &an.unwrap();
        let a = get_map_from_json(str_an, "cur_unit", cur_unit);
        let value = &a.unwrap().unwrap();
        let cur_name = value["cur_nm"].to_string().replace("\"", "");
        let buy_ex_rate: f64 = value["ttb"]
            .to_string()
            .replace("\"", "")
            .replace(",", "")
            .parse::<f64>()
            .unwrap();
        let sell_ex_rate: f64 = value["tts"]
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
                "Exchange {} dollar to 원 becomes {:?} 원 ",
                money,
                (sell_ex_rate * money as f64) as u32,
            );
        }
        Ok(Self {
            cur_unit: cur_unit.to_string(),
            buy_ex_rate: buy_ex_rate,
            sell_ex_rate: sell_ex_rate,
            cur_name: cur_name,
        })
    }
}

fn get_map_from_json(json_str: &str, key: &str, value: &str) -> Result<Option<Value>, Error> {
    let v: Value = serde_json::from_str(json_str)?;
    if let Value::Array(array) = v {
        for item in array {
            if let Value::Object(obj) = item {
                if obj.get(key) == Some(&Value::String(value.to_string())) {
                    return Ok(Some(Value::Object(obj)));
                }
            }
        }
    }

    Ok(None)
}

async fn get_request() -> Result<String, ReqwestError> {
    dotenv().ok();
    let env_api_key: String = env::var("AUTHKEY").unwrap();
    let api_key: &str = &env_api_key;

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
