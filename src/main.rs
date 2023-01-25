use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::{path::PathBuf, str::FromStr};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_sdk::{
    json::SuiJsonValue,
    types::{
        base_types::{ObjectID, SuiAddress},
        messages::Transaction,
    },
    SuiClient,
};
use sui_types::intent::Intent;
use sui_types::messages::ExecuteTransactionRequestType;

#[derive(Debug, Serialize, Deserialize)]
struct MoveFunc {
    name: String,
    description: String,
    url: Option<String>,
    content: String,
    txn_hash: Option<String>,
    owner: String,
    object_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeployError {
    code: String,
    details: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeployResponse {
    error: Option<DeployError>,
    status: i32,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// create the function
    Create {
        /// the function name
        name: String,
    },
    /// deploy the function to blockchain
    Deploy,
    /// local run
    Run,
    /// remote call the function
    Call {
        /// the function name
        name: String,
    },
    List {
        /// the functions of owner
        #[arg(short, long)]
        owner: String,

        /// the funcitons of source, can only be: db or chain
        #[arg(short, long)]
        source: String,
    },
}

#[derive(Debug, Deserialize)]
struct Config {
    basic: BasicConfig,
}

#[derive(Debug, Deserialize)]
struct BasicConfig {
    _version: String,
    name: String,
    description: String,
    owner: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Create { name }) => {
            create_action(name.clone()).await?;
        }
        Some(Commands::Deploy) => {
            deploy_action().await?;
        }
        Some(Commands::Run) => {
            println!("todo!");
        }
        Some(Commands::Call { name }) => {
            call_action(name.clone()).await?;
        }
        Some(Commands::List {
            owner: _,
            source: _,
        }) => {
            println!("todo!")
        }
        None => {}
    }

    Ok(())
}

async fn create_action(name: String) -> Result<(), anyhow::Error> {
    let path = name;
    fs::create_dir(&path)?;
    let conf = format!(
        r#"
[basic]
version = "0.0.1" 
name = "{}" # your function name, it's unique.
description = ""
owner = "0x5d547ccd49f6f35fc0dd66fb76e032e8fbf570ff" # Your sui address"#,
        &path
    );
    let conf_file = format!("{}/config.toml", &path);
    fs::write(conf_file, conf.trim_start_matches('\n'))?;

    let tpl = r#"
import * as o from "https://deno.land/x/cowsay/mod.ts"

export async function handler(payload = {}) {
    let m = o.say({
        text: "hello every one",
    })
    console.log(m)
    return m
}
"#;

    let main_file = format!("{}/main.ts", &path);
    fs::write(main_file, tpl.trim_start_matches('\n'))?;

    Ok(())
}

async fn deploy_action() -> Result<(), anyhow::Error> {
    let content = collect("main.ts".to_string()).await?;

    let conf = fs::read_to_string("config.toml")?;
    println!("{}", conf);
    let config: Config = toml::from_str(conf.as_str())?;

    println!("conf::::::::");
    let mut move_func = MoveFunc {
        name: config.basic.name,
        description: config.basic.description,
        url: None,
        content,
        owner: config.basic.owner,
        txn_hash: None,
        object_id: "".to_string(),
    };

    let object_id = mint(&move_func).await?;
    move_func.object_id = object_id;
    upload(&move_func).await?;

    Ok(())
}

async fn call_action(name: String) -> Result<(), anyhow::Error> {
    let resp = reqwest::Client::new()
        .post("https://faas3.deno.dev/api/moverun")
        .json(&serde_json::json!({ "name": name }))
        .send()
        .await?
        .text()
        .await?;
    println!("{:?}", resp);
    Ok(())
}

async fn collect(filename: String) -> Result<String, anyhow::Error> {
    let content = fs::read_to_string(filename)?;
    Ok(content)
}

async fn upload(move_func: &MoveFunc) -> Result<(), anyhow::Error> {
    let resp: serde_json::Value = reqwest::Client::new()
        .post("https://faas3.deno.dev/api/deploy")
        .json(&move_func)
        .send()
        .await?
        .json()
        .await?;
    let a = serde_json::from_value::<DeployResponse>(resp);
    println!("{:#?}", a);
    Ok(())
}

async fn mint(move_func: &MoveFunc) -> Result<String, anyhow::Error> {
    println!("mint....");
    let sui = SuiClient::new("https://fullnode.devnet.sui.io:443", None, None).await?;

    let keystore_path = default_keystore_path();
    let keystore = Keystore::File(FileBasedKeystore::new(&keystore_path)?);
    let my_address = SuiAddress::from_str(move_func.owner.as_str())?;
    let package_object_id = ObjectID::from_str("0xe911f0d207c930d299d78011e4acba66d1f1eff7")?;

    let mint_call = sui
        .transaction_builder()
        .move_call(
            my_address,
            package_object_id,
            "faas_nft",
            "mint",
            vec![],
            vec![
                SuiJsonValue::from_str(move_func.name.as_str())?,
                SuiJsonValue::from_str(move_func.description.as_str())?,
                SuiJsonValue::from_str("")?,
                SuiJsonValue::from_str(move_func.content.as_str())?,
            ],
            None,
            1000,
        )
        .await?;

    let signature = keystore.sign_secure(&my_address, &mint_call, Intent::default())?;
    let response = sui
        .quorum_driver()
        .execute_transaction(
            Transaction::from_data(mint_call, Intent::default(), signature).verify()?,
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        )
        .await?;
    assert!(response.confirmed_local_execution);

    let func_id = response
        .effects
        .unwrap()
        .created
        .first()
        .unwrap()
        .reference
        .object_id;
    println!("the object id is {:?}", func_id);
    Ok(func_id.to_string())
}

fn default_keystore_path() -> PathBuf {
    match dirs::home_dir() {
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("cannot obtain home directory path"),
    }
}
