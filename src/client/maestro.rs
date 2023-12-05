use super::config::Config;
use reqwest::RequestBuilder;
use serde::Serialize;

pub struct Maestro {
    api_key: String,
    http_client: reqwest::Client,
    pub base_url: String,
}

impl Maestro {
    pub fn new(api_key: String, network: String) -> Self {
        let cfg = Config::get_config();
        let base_url = format!(
            "https://{}.gomaestro-api.org/{}",
            &network, &cfg.client.version
        );
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .expect("Failed to create HTTP client");

        Maestro {
            api_key,
            http_client,
            base_url,
        }
    }

    async fn send_request(
        &self,
        req: RequestBuilder,
        response_body: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let req = req
            .header("Accept", "application/json")
            .header("api-key", &self.api_key)
            .build()?;

        let response = self.http_client.execute(req).await?;

        if response.status().is_success() {
            *response_body = response.text().await?;
            Ok(())
        } else {
            Err(format!("Error: {}", response.status()).into())
        }
    }

    pub async fn get(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let req = self.http_client.get(format!("{}{}", &self.base_url, url));
        let mut response_body = String::new();
        self.send_request(req, &mut response_body).await?;
        Ok(response_body)
    }

    pub async fn post<T: Serialize>(
        &self,
        url: &str,
        body: T,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let json_body = serde_json::to_string(&body)?;

        let req = self
            .http_client
            .post(format!("{}{}", &self.base_url, url))
            .header("Accept", "application/json")
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .body(json_body);

        let mut response_body = String::new();
        self.send_request(req, &mut response_body).await?;
        Ok(response_body)
    }
}
