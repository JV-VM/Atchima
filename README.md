# Atchima

**Atchima** is a 2D game project built in **Rust** using the Bevy engine.

It features a **data-driven ECS architecture**, integrating **Tiled maps**, **RON-based entity configuration**, and a modular plugin system for clean scalability.

---

## 🧠 Core Architecture

Atchima is structured around **Bevy’s ECS (Entity Component System)** and **plugin-based design**.

### 🔌 Active Plugins

The game is composed through Bevy plugins:

* `WorldManagementPlugin` → World setup, map loading, collisions, Y-sorting
* `EntityPlugin` → Entity spawning + animation setup
* `PlayerPlugin` → Player movement, input, animation switching

External plugins:

* bevy_ecs_tiled → `.tmx` map loading
* bevy_common_assets → RON config loading
* bevy_spritesheet_animation → sprite animations

---

## 📁 Project Structure

```id="realstruct"
.
├── assets/
│   ├── audios/            # Music (title.mp3)
│   ├── maps/              # Tiled maps (.tmx / .tsx)
│   ├── RON_files/         # Entity configuration (RON)
│   ├── textures/          # Character sprites
│   └── tilesets/          # Tiles grouped by biome/type
│
├── src/
│   ├── main.rs            # App entry point
│   │
│   ├── config/
│   │   └── components.rs  # ECS components + config structures
│   │
│   └── game/
│       ├── entities.rs    # Entity spawning + animation init
│       ├── player.rs      # Movement, input, camera follow
│       ├── world.rs       # Map, collisions, sorting
│       └── mod.rs
```

---

## ⚙️ Application Setup

The game initializes through Bevy:

* Default Bevy plugins (with pixel-perfect rendering)
* Tiled map support
* RON asset loading (`EntityConfig`)
* Spritesheet animation system
* Custom world management plugin

---

## 🎮 Systems Overview

### 🧍 Player System

* WASD movement
* Direction-based facing (`_UP`, `_DOWN`, etc.)
* Collision detection (AABB)
* Animation switching:

  * `IDLE`
  * `MOVING`

Animations are dynamically resolved via:

```
<entity_name>_<state>_<direction>
```

---

### 🎥 Camera System

* Follows player position
* Clamped to map boundaries
* Prevents showing outside the world

---

### 🌍 World System

* Loads map from:

```
assets/maps/first_map.tmx
```

* Spawns:

  * Tiled map
  * 2D camera

* Centers player once map is loaded

---

### 🧱 Collision System

* Collision tiles are extracted from **Tiled layers**

* Example:

  * `"Shadows"` layer → collision walls

* Each tile becomes:

  * `Collider`
  * `Collisions` marker component

---

### 🌲 Object Tagging (Trees)

* Tiles in `"trees"` layer are tagged with:

  * `Tree`
  * `YSort`

---

### 🔽 Y-Sorting System

Implements depth sorting using Y position:

* Lower Y → rendered in front
* Player uses offset for better visual alignment

---

### 🎭 Animation System

* Animations loaded from RON config
* Attached via `AnimationMapping`
* Default animation = `IDLE_DOWN`
* Automatically switches based on movement + facing

---

## 📦 Data-Driven Design

### RON Configuration

Entities are defined in:

```id="ronpath"
assets/RON_files/atchima.setup.ron
```

This includes:

* Speed
* Collider size
* Animation mappings

---

### 🗺️ Tiled Integration

Maps and tilesets are created using Tiled.

* `.tmx` → map layout
* `.tsx` → tileset definitions

---

## 🚀 Getting Started

### Requirements

* Rust (latest stable)
* Cargo

---

### Run

```bash id="runreal"
cargo run
```
## Result
---
<img width="800" height="449" alt="Atchima-game_play_testing" src="https://github.com/user-attachments/assets/beb4594a-2bce-4792-a89c-f4716bdbbd3c" />

---

## 🧩 Design Highlights

* ✔ Plugin-based architecture
* ✔ Data-driven entities (RON)
* ✔ Clean ECS separation
* ✔ Runtime animation switching
* ✔ Map-driven collision system
* ✔ Automatic Y-sorting

---

## 📌 Next Steps

* Finite State Machine (FSM) for entities
* Animation blending
* Enemy AI systems
* Interaction system
* Save/load support


---

## ✍️ Author

JV-VM
