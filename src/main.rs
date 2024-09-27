use std::error::Error;

use app::App;

mod calculations;
mod app;
mod location;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    println!("Getting your current IP address...");
    let current_ip = loop {
        match get_current_ip().await {
            Ok(ip) => break ip,
            Err(e) => {
                eprintln!("Failed to get IP address: {}. Retrying...", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    };

    let current_ip: serde_json::Value = serde_json::from_str(&current_ip).unwrap();

    println!("Getting your current geolocation...");
    let geolocation = geolocation::find(current_ip["ip"].as_str().unwrap()).unwrap();

    let mut terminal = ratatui::init();
    
    terminal.clear()?;
    let mut app = App::new(terminal, current_ip["ip"].to_string(), geolocation);

    match app.run_app() {
        Ok(_) => ratatui::restore(),
        Err(e) => eprintln!("Error: {}", e),
    };
    
    Ok(())
}

async fn get_current_ip() -> Result<String, Box<dyn Error>> {
    let request = reqwest::get("https://api64.ipify.org?format=json");
    let response = request.await;
    if response.is_ok() {
        let text = response.unwrap().text().await.unwrap();
        return Ok(text);
    } else {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to get IP address")));
    }
}