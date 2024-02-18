use serde_json::Value;
// use std::env;
use actix_web::{web, App, HttpServer, Responder, HttpRequest, middleware::Logger};
use std::error::Error;
use std::collections::HashMap;
use url::form_urlencoded;
use serde::Serialize;
use log::{info, error};

async fn get_public_ip() -> Result<String, Box<dyn Error>> {
    // let response = reqwest::Client::get("http://whatismyip.akamai.com").await?;
    let client = reqwest::Client::new();
    let response = client
        .get("http://whatismyip.akamai.com")
        .send()
        .await?;

    let public_ip = response.text().await?;
    Ok(public_ip.trim().to_string())
}


async fn get_ip_info(ip: &str) -> Result<Value, Box<dyn Error>> {
    // let client = reqwest::blocking::Client::new();
    let client = reqwest::Client::new();
    let response = client
        .get("https://apiquark.sm.cn/rest")
        .query(&[("method", "sc.number_ip_new"), ("request_sc", "shortcut_searcher::number_ip_new"), ("callback", "jsonp1"), ("q", ip)])
        .header("authority", "apiquark.sm.cn")
        .header("accept", "*/*")
        .header("accept-language", "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7")
        .header("cache-control", "no-cache")
        .header("cookie", "sm_sid=df8efaee3d12450a9bfc49f18ebc16c9; sm_ruid=df8efaee3d12450a9bfc49f18ebc16c9|||1708271519; sm_diu=df8efaee3d12450a9bfc49f18ebc16c9||1Fe0efe07acd388077|1708271519; cna=oRVZHrrnGmACAXHdKbXwDEKW; isg=BOPj1VwOghiJ004sItA7G86cciGN2HcanZwbTRVDQMKhVA92n6lTa9rCTmR_dM8S")
        .header("dnt", "1")
        .header("pragma", "no-cache")
        .header("referer", "https://quark.sm.cn/s?q=Ip&by=submit&snum=0")
        .header("sec-ch-ua", "\"Not A(Brand\";v=\"99\", \"Google Chrome\";v=\"121\", \"Chromium\";v=\"121\"")
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", "\"macOS\"")
        .header("sec-fetch-dest", "script")
        .header("sec-fetch-mode", "no-cors")
        .header("sec-fetch-site", "same-site")
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36")
        .send()
        .await?;

    let text = response.text().await?;
    let json_data = text.trim_start_matches("jsonp1(").trim_end_matches(");");
    let value: Value = serde_json::from_str(json_data)?;

    Ok(value)
}

async fn index(req: HttpRequest) -> impl Responder {
    let args = req.query_string();
    if !args.is_empty() {
        // 处理带参数的情况
        // format!("Received query parameters: {}", args)
        // let ip_address = args.get("ip".clone).unwrap();
        let query_params: HashMap<String, String> = form_urlencoded::parse(req.query_string().as_bytes())
        .into_owned()
        .collect();
        println!("{:?}", query_params);
        let ip_address = query_params.get("ip").unwrap();
        let ip_info = get_ip_info(ip_address).await.unwrap();
        format!("{}", serde_json::to_string_pretty(&ip_info).unwrap())
    } else {
        // 处理无参数的情况
        match get_public_ip().await {
            //  get ip detail
            // Ok(ip) => format!("Public IP address: {}", get_ip_info(ip.as_str()).await.unwrap()),
            //  format as json
            Ok(ip) => format!("{}", serde_json::to_string_pretty(&get_ip_info(ip.as_str()).await.unwrap()).unwrap()),
            // Ok(ip) => format!("Public IP address: {}", ip),
            Err(e) => format!("Error: {}", e),
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .map_err(|e| {
        eprintln!("Server error: {}", e);
        e
    })
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let args: Vec<String> = env::args().collect();
//     // if args.len() != 2 {
//     //     println!("Usage: {} <IP_ADDRESS>", args[0]);
//     //     return Ok(());
//     // }
//     let public_ip: String;
//     let ip_address = if args.len() > 1 {
//         &args[1]
//     } else {
//         public_ip = get_public_ip()?;
//         &public_ip
//     };

//     let ip_info = get_ip_info(ip_address)?;
//     println!("{}", serde_json::to_string_pretty(&ip_info)?);
//     Ok(())
// }
