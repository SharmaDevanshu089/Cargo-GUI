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
