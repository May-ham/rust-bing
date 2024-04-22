use reqwest::{Client, header, StatusCode};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use uuid::Uuid;
use std::path::Path;
use std::io;
use serde::{Deserialize, Serialize};

use crate::cookies;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnData {
    pub conversation_id: Option<String>,
    pub client_id: Option<String>,
    #[serde(rename = "x-sydney-encryptedconversationsignature")]
    pub x_sydney_encryptedconversationsignature: Option<String>,
    #[serde(rename = "x-sydney-conversationsignature")]
    pub x_sydney_conversationsignature: Option<String>,
}

// Define the struct for the nested "result" object.
#[derive(Serialize, Deserialize, Debug)]
pub struct ResultData {
    pub value: String,
    pub message: Option<String>,
}

// Define the struct for the top-level JSON object.
#[derive(Serialize, Deserialize, Debug)]
pub struct ConversationData {
    pub conversationId: String,
    pub clientId: String,
    pub result: ResultData,
}

// Define a custom error type
#[derive(Debug)]
pub enum CustomError {
    Forbidden,
    RequestError(reqwest::Error),
    Other(StatusCode),
    IoError(io::Error), // New variant for I/O errors
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::Forbidden => write!(f, "Access forbidden (HTTP 403). Turn on VPN and try again."),
            CustomError::RequestError(e) => write!(f, "Request error: {}", e),
            CustomError::Other(code) => write!(f, "Unexpected response status: {}", code),
            CustomError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl Error for CustomError {}

impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> Self {
        if err.status() == Some(StatusCode::FORBIDDEN) {
            CustomError::Forbidden
        } else {
            CustomError::RequestError(err)
        }
    }
}

impl From<io::Error> for CustomError {
    fn from(err: io::Error) -> Self {
        CustomError::IoError(err)
    }
}

// The standalone asynchronous function to make the request
pub async fn fetch_data() -> Result<ReturnData, CustomError> {
    println!("Starting request.");
    let client = Client::new();
    let url = "https://www.bing.com/turing/conversation/create?bundleVersion=1.1553.1";

    // (headers setup code omitted for brevity)

    // Prepare custom headers
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "accept",
        "application/json".parse().unwrap()
    );
    headers.insert(
        "Accept-Encoding",
        "gzip, deflate, br".parse().unwrap()
    );
    headers.insert(
        "accept-language",
        "en,en-GB;q=0.9,en-US;q=0.8".parse().unwrap()
    );

    let path = Path::new("cookies.json");
    let cookies = cookies::parse_cookies_from_file(&path);

    headers.insert(
        "cookie",
        cookies?.parse().unwrap(),
    );
    headers.insert(
        "referer",
        "https://www.bing.com/search?q=Bing+AI&showconv=1".parse().unwrap()
    );
    // headers.insert(
    //     "referrer-Policy",
    //     "origin-when-cross-origin".parse().unwrap()
    // );
    headers.insert(
        "Sec-Ch-Ua",
        r#""Not A(Brand";v="99", "Microsoft Edge";v="121", "Chromium";v="121""#.parse().unwrap()
    );
    headers.insert(
        "Sec-Ch-Ua-Arch",
        r#""x86""#.parse().unwrap()
    );
    headers.insert(
        "Sec-Ch-Ua-Bitness",
        r#""64""#.parse().unwrap()
    );
    headers.insert(
        "Sec-Ch-Ua-Full-Version",
        r#""121.0.2277.98""#.parse().unwrap()
    );
    headers.insert(
        "Sec-Ch-Ua-Full-Version-List",
        r#""Not A(Brand";v="99.0.0.0", "Microsoft Edge";v="121.0.2277.98", "Chromium";v="121.0.6167.139""#.parse().unwrap()
    );
    headers.insert(
        "Sec-Ch-Ua-Mobile",
        "?0".parse().unwrap()
    );
    headers.insert(
        "Sec-Ch-Ua-Model",
        r#""""#.parse().unwrap()
    );
    headers.insert(
        "Sec-Ch-Ua-Platform",
        r#""Windows""#.parse().unwrap()
    );
    headers.insert(
        "Sec-Ch-Ua-Platform-Version",
        r#""10.0.0""#.parse().unwrap()
    );
    headers.insert(
        "Sec-Fetch-Dest",
        "empty".parse().unwrap()
    );
    headers.insert(
        "Sec-Fetch-Mode",
        "cors".parse().unwrap()
    );
    headers.insert(
        "Sec-Fetch-Site",
        "same-origin".parse().unwrap()
    );
    headers.insert(
        "Sec-Ms-Gec",
        "5D193E2C72BC0C7B878AF8907A108EBC5D9E39B896B785F21A7E739A499C5168".parse().unwrap()
    );
    headers.insert(
        "Sec-Ms-Gec-Version",
        "1-121.0.2277.98".parse().unwrap()
    );
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36 Edg/121.0.0.0".parse().unwrap()
    );
    headers.insert(
        "X-Client-Data",
        "eyIxIjoiMCIsIjEwIjoiXCJ2OTVZTTdoemVOVzVUUUFKMlNTTmlsWndwdm5vQm9OWFY3ODZOZjFQZ3UwPVwiIiwiMiI6IjAiLCIzIjoiMCIsIjQiOiIzMTA4NTM4MDU3ODk4NjEzOTQiLCI1IjoiXCJ4elFhbUdxTVVMMTZidVpHeUNhZFJvV1VDL3JFVzF4OVhabHNmblhuUW93PVwiIiwiNiI6InN0YWJsZSIsIjciOiIxMTMzODcxMzY2MTUwIiwiOSI6ImRlc2t0b3AifQ==".parse().unwrap()
    );
    headers.insert(
        "X-Edge-Shopping-Flag",
        "1".parse().unwrap()
    );
    headers.insert(
        "X-Kl-Kfa-Ajax-Request",
        "Ajax_Request".parse().unwrap()
    );

    let id = Uuid::new_v4().to_string();

    headers.insert(
        "X-Ms-Client-Request-Id",
        id.parse().unwrap(),
    );
    headers.insert(
        "X-Ms-Useragent",
        "azsdk-js-api-client-factory/1.0.0-beta.1 core-rest-pipeline/1.12.3 OS/Windows".parse().unwrap()
    );

    // Make the GET request
    let response = client.get(url)
        .headers(headers)
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {

            let mut return_data = ReturnData {
                conversation_id: None,
                client_id: None,
                x_sydney_encryptedconversationsignature: None,
                x_sydney_conversationsignature: None,
            };

            for (key, value) in response.headers().iter() {
                println!("{:?}: {:?}", key, value);
                if key.as_str() == "x-sydney-encryptedconversationsignature" {
                    if let Ok(value_str) = value.to_str() {
                        return_data.x_sydney_encryptedconversationsignature = Some(value_str.to_string());
                    }
                }
                if key.as_str() == "x-sydney-conversationsignature" {
                    if let Ok(value_str) = value.to_str() {
                        return_data.x_sydney_conversationsignature = Some(value_str.to_string());
                    }
                }
            }

            let response_text = response.text().await?;

            // Parse the response text into ConversationData
            let conversation_data: ConversationData = serde_json::from_str(&response_text)
            .map_err(|e| CustomError::Other(StatusCode::INTERNAL_SERVER_ERROR))?; // Convert deserialization error to CustomError
          
            return_data.conversation_id = Some(conversation_data.conversationId);
            return_data.client_id = Some(conversation_data.clientId);

            Ok(return_data)
        }
        StatusCode::FORBIDDEN => {
            Err(CustomError::Forbidden)
        }
        status => {
            println!("Бинг наебнулся");
            Err(CustomError::Other(status))
        }
    }
}