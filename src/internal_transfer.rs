use crate::ZebedeeClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalTransferRes {
    pub success: bool,
    pub data: InternalTransferData,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalTransferTx {
    pub id: String,
    #[serde(rename = "walletId")]
    pub wallet_id: String,
    pub r#type: Option<String>,
    #[serde(rename = "totalAmount")]
    pub total_amount: String,
    pub fee: String,
    pub amount: String,
    pub description: Option<String>,
    pub status: String,
    #[serde(rename = "confirmedAt")]
    pub confirmed_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalTransferData {
    #[serde(rename = "transferId")]
    pub transfer_id: String,
    pub transaction: InternalTransferTx,
}

/// Use this struct to create a well crafted json body for your internal transfers

#[derive(Debug, Serialize, Deserialize)]
pub struct InternalTransfer {
    pub amount: String,
    #[serde(rename = "receiverWalletId")]
    pub receiver_wallet_id: String,
}

pub async fn internal_transfer(
    client: ZebedeeClient,
    internal_transfer_payload: InternalTransfer,
) -> Result<InternalTransferRes, anyhow::Error> {
    let url = format!("{}/v0/internal-transfer", client.domain);
    let resp = client
        .reqw_cli
        .post(&url)
        .header("Content-Type", "application/json")
        .header("apikey", client.apikey)
        .json(&internal_transfer_payload)
        .send()
        .await?;

    let status_code = resp.status();
    let status_success = resp.status().is_success();
    let resp_text = resp.text().await?;

    if !status_success {
        return Err(anyhow::anyhow!(
            "Error: status {}, message: {}, url: {}",
            status_code,
            resp_text,
            &url,
        ));
    }

    let resp_serialized = serde_json::from_str(&resp_text);

    let resp_seralized_2 = match resp_serialized {
        Ok(c) => c,
        Err(e) => {
            return Err(anyhow::anyhow!(
                "Was given a good status, but something failed when parsing to json\nserde parse error: {}, \ntext from API: {}\n status code: {}",
                e,
                resp_text,
                status_code
            ))
        }
    };

    Ok(resp_seralized_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_internal_transfer() {
        let apikey: String = env::var("ZBD_API_KEY").unwrap();
        let zbdenv: String =
            env::var("ZBD_ENV").unwrap_or_else(|_| String::from("https://api.zebedee.io"));
        let zebedee_client = ZebedeeClient::new().domain(zbdenv).apikey(apikey).build();

        let internal_transfer_payload = InternalTransfer {
            amount: String::from("10000"),
            receiver_wallet_id: String::from("b904ee02-ec0b-4fd4-b99f-1f2d3d0001a6"),
        };

        let r = internal_transfer(zebedee_client, internal_transfer_payload)
            .await
            .unwrap()
            .success;
        assert!(r);
    }
}