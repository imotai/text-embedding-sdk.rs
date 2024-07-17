//
// rerank.rs
// Copyright (C) 2024 imotai <imotai@imotai-ub>
// Distributed under terms of the MIT license.
//

use crate::proto::tei_proto_v1::{rerank_client::RerankClient, RerankRequest, Rank};
use tonic::{transport::Channel, Request};
use anyhow::{Result};

#[derive(Debug, Clone)]
pub struct Rerank {
    client: RerankClient<Channel>,
}


impl Rerank {
    pub fn new(channel: Channel) -> Self {
        let client = RerankClient::new(channel);
        Self { client }
    }

    pub async fn rerank(&self, query:&str, texts:&[String]) -> Result<Vec<Rank>> {
        let mut client = self.client.clone();
        let request = Request::new(RerankRequest {
            query: query.to_string(),
            texts: texts.iter().map(|x| x.to_string()).collect(),
            truncate: true,
            raw_scores: true,
            return_text: false,
            truncation_direction: 0
        });
        let response = client.rerank(request).await?;
        Ok(response.into_inner().ranks)
    }
}

#[cfg(test)]
mod tests {
	use super::*;
    use tonic::transport::Endpoint;

	#[tokio::test]
	async fn test_rerank_smoke_test() ->Result<()>{
       let ep = "http://18.138.225.70:9528";
       let rpc_endpoint = Endpoint::new(ep.to_string())?;
       let channel = rpc_endpoint.connect_lazy();
       let rerank = Rerank::new(channel);
       let result = rerank.rerank("hello world", &["hello world".to_string() , "hello world1".to_string(), "hello world2".to_string()]).await?;
       assert_eq!(result.len(), 3);
       println!("{:?}", result);
       Ok(())
	}
}
