mod model;
use reqwest::header::AUTHORIZATION;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let's see if we can fetch the weather from the backend.
    //

    let client = reqwest::Client::new();
    let params = [("username", "choyongs"), ("password", "123456789")];
    let response = client
        .post("http://localhost:3000/v1/auth")
        .form(&params)
        .send()
        .await?;

    let token_response = response
        .json::<model::Token>()
        .await?;

    println!("\nv1/auth token:\n {:?}\n", token_response);
    println!("\nv1/jwt:\n {:?}\n", token_response.AccessToken);

    let header_value = format!("Bearer {}", token_response.AccessToken);
    println!("\nheader:\n {:?}\n", header_value);

    let helloRes = client
        .get("http://localhost:3000/v1/hello")
        .header(AUTHORIZATION, header_value)
        .send()
        .await?;

    let hello_message = helloRes
        .text()
        .await?;

    println!("Hello from Node Backend:\n {:?}\n", hello_message);

    let header_value = format!("Bearer {}", token_response.AccessToken);

    let weatherRes = client
        .get("http://localhost:3000/v1/weather")
        .header(AUTHORIZATION, header_value)
        .send()
        .await?;

    let weather_message = weatherRes
        .text()
        .await?;

    println!("weather from Node Backend:\n {:?}\n", weather_message);


    Ok(())
}
