#[tokio::main]
async fn main() {
    let response = reqwest::get("https://api.openweathermap.org/data/2.5/weather?q=corvallis&appid=b98e3f089c86867862f28236d174368a&&units=imperial")
        .await
        // each response is wrapped in a `Result` type
        // we'll unwrap here for simplicity
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
}