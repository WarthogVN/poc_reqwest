/**
 * 

curl -v https://login.salesforce.com/services/oauth2/token -d "grant_type=password" -d "client_id=3MVG9fTLmJ60pJ5KRnf3pHWg8kGe.AOm0Bg2YV0nT3Juda16Vm20X78SDnSFa9FbbB8QI1N4oVQ5ExvN4_k6W" -d "client_secret=C365E3F91B6338F16C5D7B0C517A48CB1C390810A73570955C02C8D5FEF0DA16" -d "username=bruno.nguyen.huu@free.fr" -d "password=Spqr@2022!11MRVYCNVLE82qvkvcf4WeFiLJM"


curl -v https://login.salesforce.com/services/oauth2/token -d "client_id=3MVG9fTLmJ60pJ5KRnf3pHWg8kGe.AOm0Bg2YV0nT3Juda16Vm20X78SDnSFa9FbbB8QI1N4oVQ5ExvN4_k6W" -d "client_secret=C365E3F91B6338F16C5D7B0C517A48CB1C390810A73570955C02C8D5FEF0DA16" -d "username=bruno.nguyen.huu@free.fr" -d "password=Spqr@2022!11" -d "grant_type=password" 



    Key:- Content-Type  | Value: application/JSON
    Key:- Authorization  | Value: Bearer + access token. 

 */

 use std::collections::HashMap;
 use serde::{Deserialize, Serialize};
 use reqwest::header::CONTENT_TYPE;
 use reqwest::header::AUTHORIZATION;
 
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


 #[derive(Serialize, Deserialize, Debug)]
 struct LISTAPIResponse {
    tooling: String,
    metadata: String,
    eclair: String,
    folders: String,
    #[serde(rename = "prechatForms")]
    prechat_forms: String,
    #[serde(rename = "contact-tracing")]
    contact_tracing: String,
    jsonxform: String,
    chatter: String,
    tabs: String,
    #[serde(rename = "appMenu")]
    app_menu: String,
    #[serde(rename = "quickActions")]
    quick_actions: String,
    #[serde(rename = "queryAll")]
    query_all: String,
    commerce: String,
    wave: String,
    iot: String,
    analytics: String,
    search: String,
    smartdatadiscovery: String,
    identity: String,
    composite: String,
    #[serde(rename = "parameterizedSearch")]
    parameterized_search: String,
    fingerprint: String,
    theme: String,
    nouns: String,
    domino: String,
    event: String,
    #[serde(rename = "serviceTemplates")]
    service_templates: String,
    recent: String,
    connect: String,
    licensing: String,
    limits: String,
    process: String,
    dedupe: String,
    #[serde(rename = "async-queries")]
    async_queries: String,
    //async-queries: String,
    query: String,
    jobs: String,
    #[serde(rename = "match")]
    match_api: String,
    ai: String,
    localizedvalue: String,
    mobile: String,
    #[serde(rename = "emailConnect")]
    email_connect: String,
    consent: String,
    tokenizer: String,
    #[serde(rename = "compactLayouts")]
    compact_layouts: String,
    #[serde(rename = "knowledgeManagement")]
    knowledge_management: String,
    sobjects: String,
    actions: String,
    support: String,
    
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
    
    let resp_json = client.get(format!("{}/services/data/v56.0", (resp_json.instance_url)))   
    .header(CONTENT_TYPE, "application/json")
    .header(AUTHORIZATION, format!("{} {}", resp_json.token_type, resp_json.access_token))
    .send()
    .await?
//    .text().await?;
    .json::<LISTAPIResponse>()
    .await?;

    println!("{:#?}", resp_json);



    Ok(())
}
