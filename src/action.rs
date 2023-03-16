use anyhow::{Ok, Result};
use async_trait::async_trait;

#[async_trait]
pub trait Action {
    type Error;

    async fn create(&self) -> Result<(), Self::Error>;
    async fn info(&self) -> Result<(), Self::Error>;
}

#[derive(Clone, Debug)]
pub struct SupaAction {
    url: String,
    client: reqwest::Client,
}

impl SupaAction {
    pub fn new() -> Self {
        let url: String = format!("https://faas3.deno.dev/api/functions/{}", "move-did");

        Self {
            url,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Action for SupaAction {
    type Error = anyhow::Error;

    async fn create(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn info(&self) -> Result<(), Self::Error> {
        println!("this is for supa action info");
        println!("{:#?}", self);
        let resp = self
            .client
            .get(self.url.clone())
            .send()
            .await?
            .json()
            .await?;
        println!("{:#?}", resp);
        Ok(())
    }
}
