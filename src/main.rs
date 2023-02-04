use anyhow::Ok;
use clap::{Parser, Subcommand};

use serde::{Deserialize, Serialize};

use std::process::Command;
use std::{fs};
use std::{path::PathBuf, str::FromStr};
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_sdk::rpc_types::SuiData;
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
    template: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FaaSNFTMeta {
    name: String,
    description: String,
    url: String,
    content: String,
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
        // the function template
        #[arg(short, long)]
        template: String,
    },
    /// deploy the function to runtime and blockchain
    Deploy,
    /// local run
    Run,
    /// remote call the function
    Call {
        /// the function name
        name: String,
        /// the post body, it's json string
        #[arg(short, long)]
        body: String,
    },
    /// list the functions
    List {
        /// the functions of owner
        #[arg(short, long)]
        owner: String,

        /// the funcitons of source, can only be: runtime or chain
        #[arg(short, long)]
        source: String,
    },
    /// verify the runtime function, which should equal to the on-chain code.
    Verify { name: String },
}

#[derive(Debug, Deserialize)]
struct Config {
    basic: BasicConfig,
}

#[derive(Debug, Deserialize)]
struct BasicConfig {
    template: String,
    version: String,
    name: String,
    description: String,
    owner: String,
}

#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Create { name, template }) => {
            create_action(name.clone(), template.clone()).await?;
        }
        Some(Commands::Deploy) => {
            deploy_action().await?;
        }
        Some(Commands::Run) => {
            println!("This command is still WIP")
        }
        Some(Commands::Call { name, body }) => {
            call_action(name.clone(), body.clone()).await?;
        }
        Some(Commands::List {
            owner: _,
            source: _,
        }) => {
            println!("🚧 This command is still WIP!");
        }
        Some(Commands::Verify { name }) => {
            verify_action(name.clone()).await?;
        }
        None => {}
    }

    Ok(())
}

async fn create_action(name: String, template: String) -> Result<(), anyhow::Error> {
    if template.as_str() == "deno" {
        create_deno_action(name).await
    } else if template.as_str() == "node" {
        create_node_action(name).await
    } else {
        Ok(())
    }
}

async fn create_deno_action(name: String) -> Result<(), anyhow::Error> {
    let path = name;
    fs::create_dir(&path)?;
    let conf = format!(
        r#"
[basic]
template = "deno"
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

    let test_tpl = r#"
// this file is for faas3 run
import { handler } from "./main.ts";

const res = await handler();
console.log(res);    
"#;
    let test_file = format!("{}/test.ts", &path);
    fs::write(test_file, test_tpl.trim_start_matches('\n'))?;

    println!("🎉 Awesome, The [{}] function is created!", path);
    println!("🚑 change the owner to your Sui address!");

    Ok(())
}

async fn create_node_action(name: String) -> Result<(), anyhow::Error> {
    let path: String = name;
    fs::create_dir(&path)?;

    let conf = format!(
        r#"
[basic]
template = "node"
version = "0.0.1" 
name = "{}" # your function name, it's unique.
description = ""
owner = "0x5d547ccd49f6f35fc0dd66fb76e032e8fbf570ff" # Your sui address"#,
        &path
    );
    let conf_file = format!("{}/config.toml", &path);
    fs::write(conf_file, conf.trim_start_matches('\n'))?;

    let tpl = r#"
    // You can import inner sdk    
    export async function handler(payload) {
        console.log(payload)
    }
    "#;

    let main_file = format!("{}/main.mjs", &path);
    fs::write(main_file, tpl.trim_start_matches('\n'))?;

    let test_tpl = r#"
// this file is for faas3 run
import { handler } from "./main.mjs";

const res = await handler();
console.log(res);    
"#;
    let test_file = format!("{}/test.mjs", &path);
    fs::write(test_file, test_tpl.trim_start_matches('\n'))?;

    println!("🎉 Awesome, The [{}] function is created!", path);
    println!("🚑 change the owner to your Sui address!");

    Ok(())
}

async fn deploy_action() -> Result<(), anyhow::Error> {
    let conf = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(conf.as_str())?;
    println!("📖 Your Config is {:#?}", config);

    let mut name = String::default();
    if config.basic.template.as_str() == "deno" {
        name = "main.ts".to_string();
    } else if config.basic.template.as_str() == "node" {
        name = "main.mjs".to_string();
    }

    let content = collect(name).await?;

    let move_func = MoveFunc {
        name: config.basic.name,
        description: config.basic.description,
        url: None,
        content,
        owner: config.basic.owner,
        txn_hash: None,
        object_id: "".to_string(),
        template: config.basic.template,
    };

    // println!("🚀 Deploying it to blockchain...");
    // let object_id = mint(&move_func).await?;
    // move_func.object_id = object_id;

    println!("🚀 Loading it to remote db...");
    upload(&move_func).await?;

    Ok(())
}

async fn run_action() -> Result<(), anyhow::Error> {
    println!("🎬 Run function local...\n");

    let output = Command::new("deno")
        .arg("run")
        .arg("--unstable")
        .arg("-A")
        .arg("test.ts")
        .output()?;
    if !output.status.success() {
        panic!("output status error");
    }
    println!("{}", String::from_utf8(output.stdout)?);

    Ok(())
}

async fn call_action(name: String, body: String) -> Result<(), anyhow::Error> {
    let url = format!("https://faas3.deno.dev/api/functions/{}", &name);
    let resp: serde_json::Value = reqwest::Client::new().get(url).send().await?.json().await?;
    let func = serde_json::from_value::<MoveFunc>(resp)?;

    if func.template.as_str() == "deno" {
        call_deno_action(name, body).await
    } else if func.template.as_str() == "node" {
        call_node_action(name, body).await
    } else {
        panic!("not support template")
    }
}

async fn call_deno_action(name: String, body: String) -> Result<(), anyhow::Error> {
    let _url = format!("https://faas3.deno.dev/api/runner/{}", &name);
    let url = format!("http://localhost:8000/api/runner/{}", &name);

    let resp = reqwest::Client::new()
        .post(url)
        .json(&body)
        .send()
        .await?
        .text()
        .await?;
    println!("✅ Your resp is:\n {:#?}", resp);
    Ok(())
}

async fn call_node_action(name: String, body: String) -> Result<(), anyhow::Error> {
    let url = format!("https://faas3.up.railway.app/api/runner/{}", &name);

    let resp: serde_json::Value = reqwest::Client::new()
        .post(url)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    println!("✅ Your resp is:\n {:#?}", resp);
    Ok(())
}

async fn verify_action(name: String) -> Result<(), anyhow::Error> {
    println!("🚀 Verifying the function: {:?}", name);

    let url = format!("https://faas3.deno.dev/api/functions/{}", &name);
    let resp: serde_json::Value = reqwest::Client::new().get(url).send().await?.json().await?;
    let func = serde_json::from_value::<MoveFunc>(resp)?;

    println!("Function in runtime: {:#?}", func);

    let sui = SuiClient::new("https://fullnode.devnet.sui.io:443", None, None).await?;
    let object_id = ObjectID::from_str(func.object_id.as_str())?;
    let obj = sui
        .read_api()
        .get_parsed_object(object_id)
        .await?
        .object()?
        .data
        .clone()
        .try_as_move()
        .cloned();
    let fields = match obj {
        Some(v) => v.fields.to_json_value(),
        None => {
            panic!("not obj")
        }
    };
    let meta = serde_json::from_value::<FaaSNFTMeta>(fields?)?;
    println!("Function in blockchain: {:#?}", meta);

    assert!(meta.content == func.content);
    println!("{} is verified", &name);
    Ok(())
}

async fn collect(filename: String) -> Result<String, anyhow::Error> {
    let content = fs::read_to_string(filename)?;
    Ok(content)
}

async fn upload(move_func: &MoveFunc) -> Result<(), anyhow::Error> {
    let url = "https://faas3.deno.dev/api/deploy";

    let resp: serde_json::Value = reqwest::Client::new()
        .post(url)
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
    let sui = SuiClient::new("https://fullnode.devnet.sui.io:443", None, None).await?;

    let keystore_path = default_keystore_path();
    let keystore = Keystore::File(FileBasedKeystore::new(&keystore_path)?);
    let my_address = SuiAddress::from_str(move_func.owner.as_str())?;
    let package_object_id = ObjectID::from_str("0x8ea46c0da1d02a0138e513342f07accac01d44a1")?;

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
