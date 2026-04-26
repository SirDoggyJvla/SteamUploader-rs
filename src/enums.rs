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