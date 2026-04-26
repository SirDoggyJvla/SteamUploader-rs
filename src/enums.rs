
// LIBRARIES
use clap::{Subcommand}; // CLI argument parsing


// available CLI main menu options
pub mod command_options {
    pub const MANAGE_CONFIG: &'static str = "Manage mods";
    pub const INIT: &'static str = "Initialize new manifest";
    pub const UPLOAD: &'static str = "Upload content";
    pub const DELETE: &'static str = "Delete workshop item";
    pub const EXIT: &'static str = "Exit";
}

pub mod manifests_options {
    pub const ADD_MANIFEST: &'static str = "Add new manifest";
    pub const REMOVE_MANIFEST: &'static str = "Remove existing manifest";
    pub const BACK: &'static str = "Back to main menu";
}



// AVAILABLE COMMANDS FOR THE CLI
#[derive(Subcommand)]
pub enum Commands {
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

    /// Manage manifest configurations
    #[command(name = "manage-config")]
    ManageConfig {},
}