// use std::{path::Path, sync::mpsc::TryRecvError};
use steamworks;

mod steam;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Steam Uploader")]
#[command(about = "Upload mods to Steam Workshop", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new workshop item
    Create {
        /// App ID of the game to upload for
        #[arg(short, long)]
        appid: u32,
    },
    
    /// Upload content to an existing item
    Upload {
        /// App ID of the game to upload for
        #[arg(short, long)]
        appid: u32,

        /// Published file ID
        #[arg(short, long)]
        workshopid: u64,
        
        /// Path to content
        #[arg(short, long)]
        content: String,
        
        /// Path to preview image
        #[arg(short, long)]
        preview: String,

        /// Title of the item
        #[arg(short, long)]
        title: String,

        /// Description of the item
        #[arg(short, long)]
        description: String,

        /// Visibility
        /// 0 = Public, 1 = Friends Only, 2 = Private/Hidden, 3 = Unlisted
        #[arg(short, long)]
        visibility: u32,
    },
    
    /// Delete a workshop item
    Delete {
        /// Published file ID to delete
        #[arg(short, long)]
        workshopid: u64,
    },

    Test,
}



fn main() {
    // create a client pair
    let client = steamworks::Client::init().expect("Steam is not running or has not been detected");

    // get a handle to Steam's UGC module (user-generated content)
    let ugc = client.ugc();

    let args = Args::parse();
    match args.command {
        Commands::Create { appid } => steam::create::create_item(&ugc, appid),

        Commands::Upload { 
            appid, 
            workshopid, 
            content, 
            preview, 
            title, 
            description,
            visibility
        } => {
            let published_id = steamworks::PublishedFileId(workshopid);
            steam::uploader::upload_item_content(
                &ugc, appid, published_id, 
                &content, &preview, &title, 
                &description, visibility
            );
        }

        Commands::Test => {
            println!("Test command executed");
        }

        Commands::Delete { workshopid } => {
            let published_id = steamworks::PublishedFileId(workshopid);
            steam::delete::delete_item(&ugc, published_id);
        }
    }
}