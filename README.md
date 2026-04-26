# Steam Uploader Rust
Steam Uploader Rust is the successor to [Steam Uploader](https://github.com/SirDoggyJvla/Steam-Uploader), rewritten in Rust. It uses the binding library [steamworks-rs](https://github.com/Noxime/steamworks-rs) to interact with the Steamworks SDK.

The tool is a command-line application that can be used to upload and delete items on the Steam Workshop. It uses a [manifest file](#manifest-file-format) to store the necessary information for uploading items, such as the app ID, workshop ID, title, description and visibility.

## Installation
You can download the latest release from the [releases page](https://github.com/SirDoggyJvla/SteamUploader-rs/releases). Alternatively, you can build the project from source (see the [build](#build) section).

Once downloaded, store the tool in a permanent place on your computer and add the folder of the executable to your [system's PATH variable](https://stackoverflow.com/a/44272417) to be able to run it from anywhere in the terminal.

## Usage
> [!CAUTION]
> The tool cannot be run by double-clicking the executable, since it is a command-line application and needs to be run in the terminal to work properly.

You can open the CLI menu to run commands by simply running the application in the terminal without any arguments. For Windows:
```bash
SteamUploader.exe
```

And for Linux:
```bash
SteamUploader
```

You need to be in the folder of the manifest file you want to upload preferably, you can go there with the `cd` command in the terminal. For example, if your manifest file is located in `C:\MyMod\mod-manifest.json`, you can navigate to that folder with the following command:
```bash
cd C:\MyMod
```

This way, the upload command will automatically find the manifest file.

## Commands
You can get a list of all available commands with the following command:
```bash
SteamUploader --help
```

### Initialization
You can initialize a new [manifest file](#manifest-file-format) with the following command:
```bash
SteamUploader init
```

You can chose the format of the manifest file with the `--format` flag. Supported formats are `json`, `yaml`, `yml` and `toml`. The default format is `json`.
```bash
SteamUploader init --format yaml
```

### Uploading an item
To run the tool, simply execute the following command in the terminal, in the folder where the [manifest file](#manifest-file-format) is located:
```bash
SteamUploader upload
```

Alternatively, you can manually specify the manifest file:
```bash
SteamUploader upload --manifest path/to/manifest.json
```

You can pass a patch note path with the `--patchnote` flag. The patch note file can either be a text file with any extension or directly the patch note content. Patch notes in Steam use the BBCode format (see the [BBCode files](#bbcode-files) section).
```bash
SteamUploader upload --patchnote path/to/patchnote.txt
```

You can do a dry run with the `--dry-run` flag. This will read the manifest file and print the information that would be uploaded to the console, without actually uploading anything to Steam. Use this to verify if any information is wrong before doing an actual upload.
```bash
SteamUploader upload --dry-run
```

### Deleting an item
To delete an item from the Steam Workshop, you can use the following command:
```bash
SteamUploader delete --workshopid <workshop_id> --appid <app_id>
```

This does not depend on the manifest file. The `workshopid` is the ID of the item you want to delete (see [this](https://pzwiki.net/wiki/Workshop_ID)), and the `appid` is the [application ID](https://pzwiki.net/wiki/App_ID) of the game the item belongs to. For example, the app ID of Project Zomboid is `108600`.

## Manifest file format
The manifest file needs to be either a JSON, YAML or TOML file (`.json`, `.yaml`, `.yml`, `.toml`) and should be named `mod-manifest` (for example `mod-manifest.yaml`). The tool will automatically look for the manifest file in the current directory but it can also be specified with the `--manifest` flag.

You can find example manifest files in the [examples folder](test/example_manifests/). Here's the `init` command output for `mod-manifest.json`:
```json
{
  "$schema": "https://raw.githubusercontent.com/SirDoggyJvla/SteamUploader-rs/refs/heads/main/manifest_schema/mod-manifest-schema.json",
  "appid": 0,
  "content": "./Contents",
  "preview": "preview.png",
  "title": "Mod Template",
  "description": "./description.bbcode",
  "visibility": 2,
  "tags": []
}
```

| Field | Type | Description |
| --- | --- | --- |
| `$schema` | string (optional) | This field is optional and is only used for validation and autocompletion in supported editors. In Visual Studio Code, you will need to add `https://raw.githubusercontent.com/SirDoggyJvla/SteamUploader-rs` as a trusted source in your JSON schema settings to enable this feature.
| `appid` | integer | The [application ID](https://pzwiki.net/wiki/App_ID) of the game the item belongs to. For example, the app ID of Project Zomboid is `108600`. |
| `workshopid` | integer (optional) | The [workshop ID](https://pzwiki.net/wiki/Workshop_ID) of the item. This is only needed for updating existing items, not for uploading new items. When uploading a mod that didn't have a workshop ID setup, it will add the workshop ID of the newly created mod to the manifest file. |
| `content` | string | The path to the content folder that contains the files you want to upload. This can be a relative path from the manifest file or an absolute path. |
| `preview` | string | The path to the preview image that will be shown on the Steam Workshop page. This can be a relative path from the manifest file or an absolute path. |
| `title` | string | The title of the item. |
| `description` | string | The description of the item or the path to a text file that contains the description. The description should be in the [BBCode format](#bbcode-files).

## BBCode files
The [BBCode extension](https://marketplace.visualstudio.com/items?itemName=rickvansloten.bbcode) for VSCode allows for syntax highlighting of [BBCode text](https://steamcommunity.com/sharedfiles/filedetails/?id=2807121939). To use it, simply create a new file with the `.bbcode` extension and write your BBCode content in it, then simply reference that file inside your [manifest file](#manifest-file-format).

## Build
Building is done with Cargo. You can build the project with the following command:
```bash
cargo build --release
```

The built binary can be found in the `target/release` folder.

## Contact and license
The tool was created by me (discord: SimKDT) for the Project Zomboid modding community.
You can find us on the [Unofficial Modding Discord](https://pzwiki.net/wiki/PZ_Modding_Community) of Project Zomboid.

You can find the license in the [LICENSE](LICENSE) file.

## Changelogs
0.2.0
- added a CLI menu to run commands easily

0.1.2
- added $schema as a parameter of the default mod-manifest file

0.1.1
- added a check to not overwrite the mod-manifest file with the init command if it already exists

0.1.0
- Initial release