use steamworks;

mod steam;
mod manifest;
mod colors;
mod enums;

use enums::command_options;

use clap::{Parser, Subcommand};
use manifest::Manifest;
use inquire::Select;


// PARSER INFORMATION AND CLI DEFINITIONS
#[derive(Parser)]
#[command(name = "Steam Uploader")]
#[command(about = "Upload mods to Steam Workshop", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}


// AVAILABLE COMMANDS FOR THE CLI
#[derive(Subcommand)]
enum Commands {
    /// Initialize a new manifest file with default values (mod-manifest.json)
    Init {
        /// File format, default is JSON. Options: json, toml, yaml, yml
        #[arg(short, long, default_value = "json", value_parser = ["json", "toml", "yaml", "yml"])]
        format: String,
    },

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


// MAIN ENTRY POINT
fn main() {
    let args = Args::parse();
    
    let command = match args.command {
        Some(cmd) => cmd,
        None => show_interactive_menu(),
    };
    
    execute_command(command);
}


// INTERACTIVE MENU
// used when no command is provided via CLI arguments
// shows a menu to select the command from
fn show_interactive_menu() -> Commands {
    let options = vec![
        command_options::INIT,
        command_options::UPLOAD,
        command_options::DELETE,
    ];

    let selection = Select::new("What would you like to do?", options)
        .prompt()
        .expect("Failed to read selection");

    match selection {
        // initialize a new manifest file
        command_options::INIT => {
            // ask for the format of the manifest file
            let format = inquire::Select::new(
                "Choose manifest format:",
                vec!["json", "toml", "yaml"],
            )
            .prompt()
            .expect("Failed to read format");

            // run
            Commands::Init {
                format: format.to_string(),
            }
        }


        // upload content to steam workshop
        command_options::UPLOAD => {
            // ask for custom manifest file path
            let manifest_path = inquire::Text::new("Manifest file path (leave empty for default):")
                .prompt()
                .ok()
                .and_then(|p| if p.is_empty() { None } else { Some(p) });

            // ask for patch note file path (optional)
            let patchnote = inquire::Text::new("Patch note file path (optional, leave empty to skip):")
                .prompt()
                .ok()
                .and_then(|p| if p.is_empty() { None } else { Some(p) });

            // ask if they want to do a dry run (default: no)
            let dry_run = inquire::Confirm::new("Perform a dry run (no actual upload)?")
                .with_default(false)
                .prompt()
                .unwrap_or(false);

            // run
            Commands::Upload {
                patchnote,
                manifest: manifest_path,
                dry_run,
            }
        }


        // delete a workshop item
        command_options::DELETE => {
            // ask for the workshop ID to delete
            let workshopid = inquire::Text::new("Workshop ID to delete:")
                .prompt()
                .expect("Failed to read input")
                .parse::<u64>()
                .expect("Invalid Workshop ID");

            // ask for the app ID
            let appid = inquire::Text::new("App ID:")
                .prompt()
                .expect("Failed to read input")
                .parse::<u32>()
                .expect("Invalid App ID");

            // run
            Commands::Delete { workshopid, appid }
        }

        // this should never happen since we control the options, but just in case
        _ => unreachable!(),
    }
}



// HANDLING OF THE SELECTED COMMAND
fn execute_command(command: Commands) {
    match command {
        // initialize a new manifest file with default values
        Commands::Init { format } => {
            Manifest::init(&format);
        }


        // handle the upload command
        Commands::Upload { patchnote, manifest: manifest_path, dry_run } => {
            match Manifest::load_default(manifest_path) {
                Ok(mut manifest) => {
                    let appid = manifest.appid;
                    let (client, ugc) = steam::load::load_steam(appid);

                    // verify that the workshop ID was provided
                    // if not, then we create a new item and update the manifest file
                    let published_id = if let Some(workshopid) = manifest.workshopid {
                        // already have a workshop ID, just upload
                        colors::info(&format!("Found existing workshopid in manifest: {}", workshopid));
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

                    let patchnote = match manifest.try_read_file(patchnote.unwrap_or_default()) {
                        Ok(note) => Some(note),
                        Err(e) => {
                            colors::error(&format!("Error reading patch note: {}", e));
                            return;
                        }
                    };

                    // upload the content
                    colors::info("Uploading content to Steam Workshop...");
                    steam::uploader::upload_item_content(
                        &ugc,
                        manifest.appid,
                        published_id,
                        &content_path,
                        &preview_path,
                        &manifest.title,
                        &description,
                        manifest.visibility,
                        manifest.tags,
                        patchnote.as_deref(),
                        dry_run,
                    );
                }
                Err(e) => colors::error(&format!("Error loading manifest: {}", e)),
            }
        }


        // here we delete the workshop item provided by the user. Steam's API handles basically everything else,
        // we don't need to verify if it exists or if the user is the owner, etc.
        Commands::Delete { workshopid, appid } => {
            let (_, ugc) = steam::load::load_steam(appid);
            let published_id = steamworks::PublishedFileId(workshopid);
            steam::delete::delete_item(&ugc, published_id);
        }
    }
}