//
// embed.rs
// Copyright (C) 2024 imotai <imotai@imotai-ub>
//

use crate::proto::tei_proto_v1::{embed_client::EmbedClient, EmbedRequest};
use tonic::{transport::Channel, Request};
use anyhow::{Result};
use std::default::Default;

#[derive(Debug, Clone)]
pub struct Embed {
    client: EmbedClient<Channel>,
}

// TruncateDirection is the direction of truncation
#[derive(Debug, Clone)]
pub enum TruncateDirection {
    Right = 0,
    Left,
}

impl Default for TruncateDirection {
    fn default() -> Self {
        TruncateDirection::Right
    }
}

#[derive(Debug, Clone)]
pub struct EmbedOption {
    pub truncate: bool,
    pub normalize: bool,
    pub truncation_direction: TruncateDirection,
}

impl Default for EmbedOption {
    fn default() -> Self {
        EmbedOption {
            truncate: false,
            normalize: false,
            truncation_direction: TruncateDirection::default()
        }
    }
}

impl EmbedOption {
    pub fn new(truncate: bool, normalize: bool, truncation_direction: TruncateDirection) -> Self {
        EmbedOption {
            truncate,
            normalize,
            truncation_direction,
        }
    }
}

impl Embed {

    pub fn new(channel: Channel) -> Self {
        let client = EmbedClient::new(channel);
        Self { client }
    }

    pub async fn embed(&self, text: &str, option:Option<EmbedOption>) -> Result<Vec<f32>> {
        let mut client = self.client.clone();
        let option = if let Some(option) = option {
            option
        } else {
            EmbedOption::default()
        };
        let request = Request::new(EmbedRequest {
            inputs: text.to_string(),
            truncate: option.truncate,
            normalize: option.normalize,
            truncation_direction: option.truncation_direction as i32,
            prompt_name: None
        });
        let response = client.embed(request).await?;
        Ok(response.into_inner().embeddings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::transport::Endpoint;
    // test embed method
    #[tokio::test]
    async fn test_embed_smoke_test() ->Result<()>{
       let ep = "http://18.138.225.70:9527";
       let rpc_endpoint = Endpoint::new(ep.to_string())?;
       let channel = rpc_endpoint.connect_lazy();
       let embed = Embed::new(channel);
       let result = embed.embed("hello world", None).await?;
       assert_eq!(result.len(), 768);
       println!("{:?}", result);
       Ok(())
    }

}
