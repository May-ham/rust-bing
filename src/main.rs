mod request;
mod cookies;
mod websockets;

#[tokio::main]
async fn main() {
    let websocket_struct = match request::fetch_data().await {
        Ok(return_data) => {
            println!("Data fetched successfully.");
            println!("Struct: {:?}", return_data);
            return_data
        }
        Err(e) => {
            println!("Error from HTTP request: {}", e);
            // If you want to crash the program on error, use:
            panic!("Program terminated due to error: {}", e);
            // Or if you prefer a cleaner exit without panic:
            // std::process::exit(1);
        }
    };

    match websockets::connect_to_websocket(websocket_struct).await {
        Ok(_) => {
            println!("WebSocket connected successfully.");
            // Handle successful WebSocket connection here
        }
        Err(e) => {
            println!("Error: {}", e);
            // If you want to crash the program on error, use:
            panic!("Program terminated due to error: {}", e);
            // Or if you prefer a cleaner exit without panic:
            // std::process::exit(1);
        }
    }
}