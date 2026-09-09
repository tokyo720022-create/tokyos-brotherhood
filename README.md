# Tokyo's Brotherhood

A Linux-first, open-source anime and manga application built with Rust, GTK4, and Libadwaita.

Tokyo's Brotherhood is designed to be a local-first media client inspired by applications such as Mihon, while developing its own anime video player, manga reader, extension system, library, history, downloads, and tracking features.

## Project Status

🚧 Early development

The project is currently in the foundation and architecture phase.

### Currently implemented

- Rust application foundation
- GTK4 + Libadwaita UI
- Native Linux application window
- Application navigation shell
- Library interface
- User-defined library categories
- History interface
- Cover-ready media model
- Source abstraction
- Source manager
- Test source
- Extension repository manifest model
- Extension repository UI
- Git/GitHub project workflow

## Planned Features

### Anime

- Anime discovery
- Multiple source extensions
- Episode listings
- Advanced video playback
- Multiple audio tracks
- Advanced subtitle support
- ASS/SSA subtitle rendering
- Playback speed control
- Intro/outro skipping
- Resume playback
- Screenshot support
- Hardware-accelerated playback where available

### Manga

- Manga discovery
- Multiple source extensions
- Chapter listings
- High-quality image rendering
- RTL reading
- LTR reading
- Webtoon mode
- Continuous scrolling
- Double-page reading
- Zoom and page controls
- Offline reading
- Intelligent image caching

### Library

- Anime and manga library
- Custom categories
- Multiple categories per title
- Favorites
- Reading/watching status
- Progress tracking
- Local-first storage

### History

- Anime watch history
- Manga reading history
- Episode/chapter progress
- Resume position
- Source information
- Cached cover artwork

### Extensions

Tokyo's Brotherhood is designed around a source/extension architecture.

Extensions will eventually provide:

- Search
- Media metadata
- Cover artwork
- Anime episodes
- Manga chapters
- Stream/page resolution

The main application will remain separate from individual content sources.

### Tracking

Planned integrations:

- AniList
- MyAnimeList

Tracking will be used for metadata, library synchronization, and progress synchronization.

## Architecture

The long-term architecture is planned around separate components:

```text
Tokyo's Brotherhood
│
├── Application UI
│
├── Anime Engine
│   └── Video Player
│
├── Manga Engine
│   └── Manga Reader
│
├── Source / Extension System
│
├── Media Model
│
├── Library
│
├── History
│
├── Downloads
│
├── Local Database
│
└── Tracking
    ├── AniList
    └── MyAnimeList


    Design Principles
Local-first
User library, history, settings, cache, and progress should primarily live on the user's own device.
No mandatory backend
Tokyo's Brotherhood is intended to work as a client application. It should not require a proprietary cloud backend for normal operation.
Modular
Anime, manga, tracking, sources, reader, and player components should remain independently maintainable.
Open source
The project is intended to remain open source and transparent.
Learn and improve
Tokyo's Brotherhood takes inspiration from existing open-source anime and manga applications while developing its own architecture and features.

Technology
Current stack:
Rust
GTK4
Libadwaita
Cargo
Git
Planned technologies may include:
SQLite
FFmpeg
libmpv
WebAssembly-based extensions
AniList API
MyAnimeList API

Development

Clone the repository:

git clone https://github.com/tokyo720022-create/tokyos-brotherhood.git
cd tokyos-brotherhood

Build:
cargo build
Run:
cargo run
Check:
cargo check
Repository Structure
src/
├── main.rs
├── app.rs
├── media.rs
├── library.rs
├── history.rs
├── extensions.rs
└── sources/
    ├── mod.rs
    ├── test_source.rs
    └── repository.rs

Disclaimer
Tokyo's Brotherhood is a software project for interacting with media sources and metadata services.
Users are responsible for complying with the laws and terms applicable to the sources and content they access.

License

License: To be decided.


