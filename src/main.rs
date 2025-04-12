use reqwest;
use serde_json::Value;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 環境変数の読み込み
    dotenv().ok();
    
    // APIキーの取得
    let api_key = env::var("OPENWEATHER_API_KEY").expect("OPENWEATHER_API_KEY not set");
    
    // 都市名（東京）
    let city = "Tokyo";
    
    // APIエンドポイント
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );
    
    // クライアントの作成
    let client = reqwest::Client::new();
    
    // APIリクエスト
    let response = client.get(&url).send().await?;
    let weather_data: Value = response.json().await?;
    
    // 天気情報の取得
    let temp = weather_data["main"]["temp"].as_f64().unwrap_or(0.0);
    let humidity = weather_data["main"]["humidity"].as_i64().unwrap_or(0);
    let description = weather_data["weather"][0]["description"].as_str().unwrap_or("不明");
    
    // ニュース形式で表示
    println!("\n=== 天気予報ニュース ===");
    println!("日時: {}", chrono::Local::now().format("%Y-%m-%d %H:%M"));
    println!("都市: {}", city);
    println!("気温: {:.1}°C", temp);
    println!("湿度: {}%", humidity);
    println!("天気: {}", description);
    println!("=====================\n");
    
    Ok(())
} 