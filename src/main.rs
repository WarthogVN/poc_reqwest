/**
 * 

curl -v https://login.salesforce.com/services/oauth2/token -d "grant_type=password" -d "client_id=3MVG9fTLmJ60pJ5KRnf3pHWg8kGe.AOm0Bg2YV0nT3Juda16Vm20X78SDnSFa9FbbB8QI1N4oVQ5ExvN4_k6W" -d "client_secret=C365E3F91B6338F16C5D7B0C517A48CB1C390810A73570955C02C8D5FEF0DA16" -d "username=bruno.nguyen.huu@free.fr" -d "password=Spqr@2022!11MRVYCNVLE82qvkvcf4WeFiLJM"


curl -v https://login.salesforce.com/services/oauth2/token -d "client_id=3MVG9fTLmJ60pJ5KRnf3pHWg8kGe.AOm0Bg2YV0nT3Juda16Vm20X78SDnSFa9FbbB8QI1N4oVQ5ExvN4_k6W" -d "client_secret=C365E3F91B6338F16C5D7B0C517A48CB1C390810A73570955C02C8D5FEF0DA16" -d "username=bruno.nguyen.huu@free.fr" -d "password=Spqr@2022!11" -d "grant_type=password" 



 */

 use std::collections::HashMap;
 use serde::{Deserialize, Serialize};
 use reqwest::header::CONTENT_TYPE;
 
 
 #[derive(Serialize, Deserialize, Debug)]
 struct GETAPIResponse {
     origin: String,
 }
 
 #[derive(Serialize, Deserialize, Debug)]
 struct JSONResponse {
     json: HashMap<String, String>,
 }
 

 #[derive(Serialize, Deserialize, Debug)]
 struct AUTHAPIResponse {
    access_token: String,
    instance_url: String,
    id: String,
    token_type: String,
    issued_at: String,
    signature: String,
 }

 #[tokio::main]
 async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, poc_reqwest!");

    // Create a Map of string key-value pairs 
    // to represent the body payload
    let mut map = HashMap::new();
    map.insert("grant_type", "password");
    map.insert("client_id", "3MVG9fTLmJ60pJ5KRnf3pHWg8kGe.AOm0Bg2YV0nT3Juda16Vm20X78SDnSFa9FbbB8QI1N4oVQ5ExvN4_k6W");
    map.insert("client_secret", "C365E3F91B6338F16C5D7B0C517A48CB1C390810A73570955C02C8D5FEF0DA16");
    map.insert("username", "bruno.nguyen.huu@free.fr");
    map.insert("password", "Spqr@2022!11MRVYCNVLE82qvkvcf4WeFiLJM");


    let params = [("grant_type", "password")
    , ("client_id", "3MVG9fTLmJ60pJ5KRnf3pHWg8kGe.AOm0Bg2YV0nT3Juda16Vm20X78SDnSFa9FbbB8QI1N4oVQ5ExvN4_k6W")
    , ("client_secret", "C365E3F91B6338F16C5D7B0C517A48CB1C390810A73570955C02C8D5FEF0DA16")
    , ("username", "bruno.nguyen.huu@free.fr")
    , ("password", "Spqr@2022!11MRVYCNVLE82qvkvcf4WeFiLJM")
    ];


    // - Create a new client which is re-used between requests
    let client = reqwest::Client::new();

// - Doing a POST request
// - Parse the response to the "JSONResponse" struct
let resp_json = client.post("https://login.salesforce.com/services/oauth2/token")
    //.header(CONTENT_TYPE, "application/json")
    .form(&params)
    .send()
    .await?
//    .text().await?;
    .json::<AUTHAPIResponse>()
    .await?;

println!("{:#?}", resp_json);

    Ok(())
}
