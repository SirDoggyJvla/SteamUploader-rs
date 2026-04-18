// use std::{path::Path, sync::mpsc::TryRecvError};
use steamworks;

mod steam;
mod manifest;

use clap::{Parser, Subcommand};
use manifest::Manifest;

#[derive(Parser)]
#[command(name = "Steam Uploader")]
#[command(about = "Upload mods to Steam Workshop", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new workshop item (loads from manifest file)
    Create {
        /// Optional path to manifest file
        #[arg(short, long)]
        manifest: Option<String>,
    },
    
    /// Upload content to an item (creates if workshopid not in manifest)
    Upload {
        /// Optional patch note to include with the upload
        #[arg(short, long)]
        patchnote: Option<String>,

        /// Optional path to manifest file
        #[arg(short, long)]
        manifest: Option<String>,
    },
    
    /// Delete a workshop item
    Delete {
        /// Published file ID to delete
        #[arg(short, long)]
        workshopid: u64,
    },
}



fn main() {
    // create a client pair
    let client = steamworks::Client::init().expect("Steam is not running or has not been detected");

    // get a handle to Steam's UGC module (user-generated content)
    let ugc = client.ugc();

    let args = Args::parse();
    match args.command {
        Commands::Create { manifest: manifest_path } => {
            match Manifest::load_default(manifest_path) {
                Ok(manifest) => {
                    match steam::create::create_item(&client, &ugc, manifest.appid) {
                        Ok(published_id) => {
                            println!("Successfully created workshop item: {:?}", published_id);
                            println!("Add the following to your manifest file:");
                            println!("workshopid = {}", published_id.0);
                        }
                        Err(e) => eprintln!("Failed to create workshop item: {}", e),
                    }
                }
                Err(e) => eprintln!("Error loading manifest: {}", e),
            }
        }

        Commands::Upload { patchnote, manifest: manifest_path } => {
            match Manifest::load_default(manifest_path) {
                Ok(mut manifest) => {
                    // verify that the workshop ID was provided
                    // if not, then we create a new item and update the manifest file
                    let published_id = if let Some(workshopid) = manifest.workshopid {
                        // already have a workshop ID, just upload
                        steamworks::PublishedFileId(workshopid)
                    } else {
                        // no workshop ID, create one
                        println!("No workshopid found in manifest. Creating new workshop item...");
                        match steam::create::create_item(&client, &ugc, manifest.appid) {
                            Ok(id) => {
                                println!("Created workshop item: {:?}", id);
                                // update manifest with new workshopid and save it to the source file
                                match manifest.save_with_id_to_source(id.0) {
                                    Ok(_) => println!("Updated manifest with new workshopid"),
                                    Err(e) => eprintln!("Warning: Could not update manifest file: {}", e),
                                }
                                id
                            }
                            Err(e) => {
                                eprintln!("Error creating workshop item: {}", e);
                                return;
                            }
                        }
                    };

                    // upload the content
                    steam::uploader::upload_item_content(
                        &ugc,
                        manifest.appid,
                        published_id,
                        &manifest.content,
                        &manifest.preview,
                        &manifest.title,
                        &manifest.description,
                        manifest.visibility,
                        patchnote.as_deref(),
                    );
                }
                Err(e) => eprintln!("Error loading manifest: {}", e),
            }
        }


        // Here we delete the workshop item provided by the user. Steam's API handles basically everything else,
        // we don't need to verify if it exists or if the user is the owner, etc.
        Commands::Delete { workshopid } => {
            let published_id = steamworks::PublishedFileId(workshopid);
            steam::delete::delete_item(&ugc, published_id);
        }
    }
}