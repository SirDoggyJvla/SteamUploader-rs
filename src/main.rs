// use std::{path::Path, sync::mpsc::TryRecvError};
use steamworks;

mod steam;
mod manifest;
mod colors;

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
    /// Upload content to an item (creates if workshopid not in manifest)
    Upload {
        /// Optional patch note to include with the upload
        #[arg(short, long)]
        patchnote: Option<String>,

        /// Optional path to manifest file
        #[arg(short, long)]
        manifest: Option<String>,

        /// Optional flag to make a dry run (no actual upload, just print what would be uploaded)
        #[arg(short, long)]
        dry_run: bool,
    },
    
    /// Delete a workshop item
    Delete {
        /// Published file ID to delete
        #[arg(short, long)]
        workshopid: u64,

        /// App ID
        #[arg(short, long)]
        appid: u32,
    },
}



fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Upload { patchnote, manifest: manifest_path, dry_run } => {
            match Manifest::load_default(manifest_path) {
                Ok(mut manifest) => {
                    let appid = manifest.appid;
                    let (client, ugc) = steam::load::load_steam(appid);

                    // verify that the workshop ID was provided
                    // if not, then we create a new item and update the manifest file
                    let published_id = if let Some(workshopid) = manifest.workshopid {
                        // already have a workshop ID, just upload
                        steamworks::PublishedFileId(workshopid)
                    } else {
                        // no workshop ID, create one
                        colors::warning("No workshopid found in manifest. Creating new workshop item...");
                        match steam::create::create_item(&client, &ugc, manifest.appid, dry_run) {
                            Ok(id) => {
                                colors::success(&format!("Created workshop item: {:?}", id));
                                // update manifest with new workshopid and save it to the source file
                                if dry_run {
                                    colors::info("Dry run enabled. Skipping manifest update with new workshop ID.");
                                } else {
                                    match manifest.save_with_id_to_source(id.0) {
                                        Ok(_) => colors::info("Updated manifest with new workshopid"),
                                        Err(e) => colors::error(&format!("Warning: Could not update manifest file: {}", e)),
                                    }
                                }
                                id
                            }
                            Err(e) => {
                                colors::error(&format!("Error creating workshop item: {}", e));
                                return;
                            }
                        }
                    };

                    // resolve content and preview paths relative to the manifest file
                    let content_path = match manifest.get_content_path() {
                        Ok(path) => path.to_string_lossy().to_string(),
                        Err(e) => {
                            colors::error(&format!("Error resolving content path: {}", e));
                            return;
                        }
                    };
                    let preview_path = match manifest.get_preview_path() {
                        Ok(path) => path.to_string_lossy().to_string(),
                        Err(e) => {
                            colors::error(&format!("Error resolving preview path: {}", e));
                            return;
                        }
                    };
                    let description = match manifest.get_description() {
                        Ok(desc) => desc,
                        Err(e) => {
                            colors::error(&format!("Error reading description: {}", e));
                            return;
                        }
                    };

                    // upload the content
                    steam::uploader::upload_item_content(
                        &ugc,
                        manifest.appid,
                        published_id,
                        &content_path,
                        &preview_path,
                        &manifest.title,
                        &description,
                        manifest.visibility,
                        patchnote.as_deref(),
                        dry_run,
                    );
                }
                Err(e) => colors::error(&format!("Error loading manifest: {}", e)),
            }
        }


        // Here we delete the workshop item provided by the user. Steam's API handles basically everything else,
        // we don't need to verify if it exists or if the user is the owner, etc.
        Commands::Delete { workshopid, appid } => {
            let (_, ugc) = steam::load::load_steam(appid);
            let published_id = steamworks::PublishedFileId(workshopid);
            steam::delete::delete_item(&ugc, published_id);
        }
    }
}