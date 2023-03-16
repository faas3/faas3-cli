use anyhow::Ok;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

use crate::action::{Action, SupaAction};

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

#[derive(Debug, Deserialize)]
struct Config {
    basic: BasicConfig,
}

#[derive(Debug, Deserialize)]
struct BasicConfig {
    template: String,
    #[allow(unused)]
    version: String,
    name: String,
    description: String,
    owner: String,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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
        owner: Option<String>,

        /// the functions in template
        #[arg(short, long)]
        template: Option<String>,
    },
    /// show the function info
    Info {
        /// the function name
        name: String,
    },
    /// verify the runtime function, which should equal to the on-chain code.
    Verify { name: String },
}

pub async fn create_action(name: String, template: String) -> Result<(), anyhow::Error> {
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

pub async fn deploy_action() -> Result<(), anyhow::Error> {
    let conf = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(conf.as_str())?;
    println!("📖 Your Config is {:#?}", config);

    let mut name = String::default();
    if config.basic.template.as_str() == "deno" {
        name = "main.ts".to_string();
    } else if config.basic.template.as_str() == "node" {
        name = "main.mjs".to_string();
    }

    let content = fs::read_to_string(name)?;
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

    upload(&move_func).await?;

    Ok(())
}

pub async fn call_action(name: String, body: String) -> Result<(), anyhow::Error> {
    let url = format!("https://faas3.deno.dev/api/runner/{}", &name);

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

pub async fn verify_action(name: String) -> Result<(), anyhow::Error> {
    println!("🚀 Verifying the function: {:?}", name);
    Ok(())
}

pub async fn list_action(
    owner: Option<String>,
    template: Option<String>,
) -> Result<(), anyhow::Error> {
    let url = "https://faas3.deno.dev/api/functions";
    let resp: serde_json::Value = reqwest::Client::new().get(url).send().await?.json().await?;
    let func_list = serde_json::from_value::<Vec<MoveFunc>>(resp)?;
    func_list
        .iter()
        .filter(|item| match owner.clone() {
            Some(o) => o == item.owner,
            None => true,
        })
        .filter(|item| match template.clone() {
            Some(t) => t == item.template,
            None => true,
        })
        .map(|item| (item.name.clone(), item.template.clone(), item.owner.clone()))
        .for_each(|item| {
            println!("{:#?}", item);
        });
    Ok(())
}

pub async fn info_action(name: String) -> Result<(), anyhow::Error> {
    println!("🚀 The {:?} detail...", name);
    let url = format!("https://faas3.deno.dev/api/functions/{}", &name);
    let resp: serde_json::Value = reqwest::Client::new().get(url).send().await?.json().await?;
    let func = serde_json::from_value::<MoveFunc>(resp)?;
    println!("Function in runtime:\n {:#?}", func);

    let action = SupaAction::new();
    action.info().await?;

    Ok(())
}

#[allow(unused)]
async fn run_deno_action() -> Result<(), anyhow::Error> {
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
