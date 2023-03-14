use anyhow::Ok;
use clap::Parser;
use cli::{
    call_action, create_action, deploy_action, info_action, list_action, verify_action, Cli,
    Commands,
};

mod cli;

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
        Some(Commands::List { owner, template }) => {
            list_action(owner.clone(), template.clone()).await?;
        }
        Some(Commands::Verify { name }) => {
            verify_action(name.clone()).await?;
        }
        Some(Commands::Info { name }) => {
            info_action(name.clone()).await?;
        }
        None => {}
    }

    Ok(())
}
