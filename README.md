# Atchima

**Atchima** is a game project built in **Rust** using the Bevy engine.
It focuses on a **data-driven ECS architecture**, external configuration, and scalable world design using Tiled maps.

The project is structured to keep **systems, components, and assets cleanly separated**, making it easy to expand gameplay features over time.

---

## 📁 Project Structure

```
.
├── assets/
│   ├── audios/            # Music and sound effects
│   ├── maps/              # Tiled maps (.tmx / .tsx)
│   ├── RON_files/         # External configuration (RON format)
│   ├── textures/          # Sprites and textures
│   └── tilesets/          # Tileset images organized by theme
│
├── src/
│   ├── main.rs            # Bevy App entry point
│   │
│   ├── config/            # ECS component definitions
│   │   ├── components.rs
│   │   └── mod.rs
│   │
│   └── game/              # Gameplay logic (Bevy systems)
│       ├── entities.rs    # Entity spawning
│       ├── player.rs      # Player systems & behavior
│       ├── world.rs       # Map/world systems
│       └── mod.rs
│
├── Cargo.toml
└── README.md
```

---

## 🧠 Architecture (Bevy ECS)

Atchima is built around **Bevy’s ECS (Entity Component System)**:

### 🧩 Components (`config/`)

* Pure data structures
* Attached to entities
* Examples:

  * Position
  * Movement data
  * Player state

---

### ⚙️ Systems (`game/`)

* Functions that operate on components
* Run every frame via Bevy’s scheduler
* Examples:

  * Player movement
  * World updates
  * Entity spawning

---

### 🧱 Entities

* Created in `entities.rs`
* Composed dynamically from components
* Represent players, world objects, etc.

---

### 🌍 World

* Managed in `world.rs`
* Integrates **Tiled maps** into the Bevy world
* Handles environment and spatial logic

---

## 🎮 Features

* ⚡ Built with **Bevy ECS**
* 🗺️ Tiled map support (`.tmx`, `.tsx`)
* 📦 External configuration using **RON**
* 🎭 Modular entity system
* 🧍 Player-specific systems
* 🎨 Organized asset pipeline (textures, tilesets)
* 🔊 Audio integration

---

## ⚙️ Data-Driven Design

### RON Configuration

Game setup is defined externally:

```
assets/RON_files/atchima.setup.ron
```

This allows:

* Tweaking behavior without recompiling
* Cleaner separation between logic and data

---

### 🗺️ Maps (Tiled)

Maps are created using Tiled and stored in:

```
assets/maps/
```

Tilesets are grouped under:

```
assets/tilesets/
```

---

## 🚀 Getting Started

### Prerequisites

* Rust (latest stable)
* Cargo

---

### Run the project

```bash
cargo run
```

---

## 🧩 Design Philosophy

* Favor **composition over inheritance**
* Keep systems **small and focused**
* Use **data-driven patterns** wherever possible
* Maintain **clear separation of concerns**

---

## 📌 Future Improvements

* Animation system (sprite sheets + state handling)
* Finite State Machine for entities
* Physics / collision systems
* Save & load support
* Debug tools and editor utilities

---

## ✍️ Author

JV-VM
