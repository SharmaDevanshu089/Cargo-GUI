# Cargo GUI
 
 This is a Unofficial Project of mine to create a proper , industry standard (in future i hope) GUI tool for Cargo, Official Package Manager. Im not a professional(Wanna Be) but i have been using Rust for a year now and i think i can make a good GUI tool for it.

 ## Planned Feature
  - Scan All Projects in a User Drive
  - Read Cargo Toml Files and list down all Dependencies
  - Dependancy Management
  - Ability to Create New Projects
  - Ability To manage old/new projects and publishing
  - Cross Platform is planned (Windows, Linux, MacOS) 
  > Note: I have never used macOS and dont know much about their ecosystem and design lanugage, If you are MacOS user , very much welcome.


## Very Far Fetched Feature (I may or may not)
  - Ability to Go Cross Ecosystem

  ## Tech Stack
   - SvelteKit
   - Tauri 
   - Rust (OFC)

> Out of all Projects , this is the one i am thinking to manage long term and keep updating until i can. (I Hope So).

## Dev Documentation: 

### Contribution :
As of right now (19 June 2026) i dont think anyone will ever use this or build this but if you see this , every PR is welcome. Every hand matters.

### How to Build

#### Prerequisite 
    - Rust (Cargo & Rustc)
    - NodeJs (For npm)

#### Installation Steps
    1. Clone the repository
    2. Run `npm install` to install dependencies
    3. Run `npm run dev` to start the development server
    4. Run `npm run tauri build` to build the project (This will create an executable in `target/release` directory)

## TODO: 
 -> Get better High Contrast Icon (Current One is temporary).


### Versions

 #### Pre 0.1.0
 -> Implemented a custom Tauri titlebar component styled with Catppuccin Mocha colors and modern SVG window control icons, integrated via root SvelteKit +layout (20 June 2026).
 -> Replaced the third-party `@el3um4s/svelte-titlebar` library with a custom implementation.
 -> Started working on the Sidebar Navigation (Projects, Dependencies, Create Project, Settings).
 -> Implemented a pre-launch Rust configuration manager (24 June 2026) that initializes a default configuration file (`app.json`) in the application's AppData directory (`scanDirectory` defaults to the user's Home directory, and `newProjectDirectory` defaults to the Documents directory). It reads and validates the configuration on startup and guarantees that existing user preferences are never overwritten.

## Configuration

Cargo GUI stores its configuration in the system's standard `AppData` directory to persist preferences across launches.

### File Location
- **Windows**: `C:\Users\<username>\AppData\Roaming\Cargo GUI\app.json`
- **Linux**: `/home/<username>/.config/Cargo GUI/app.json`
- **macOS**: `/Users/<username>/Library/Application Support/Cargo GUI/app.json`

### Config File Format

The configuration file is a simple JSON file containing the following fields:
*   `scanDirectory`: The directory that Cargo GUI will scan for Rust projects on startup (defaults to the user's Home directory).
*   `newProjectDirectory`: The default folder where new Cargo projects will be created (defaults to the user's Documents directory).

Example `app.json`:
```json
{
  "scanDirectory": "C:\\Users\\sharm",
  "newProjectDirectory": "C:\\Users\\sharm\\Documents"
}
```

> [!IMPORTANT]
> **Never Overwritten**: Cargo GUI checks for this file during pre-launch startup. If the file exists, it will load it directly and will **never** overwrite your custom directories. If you want to reset your configuration, you can safely delete this file and it will be recreated with defaults on the next launch.
