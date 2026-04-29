# Superhero Platformer Game Spec (Rust + Bevy)

## 1. Overview

A simple, colorful 2D side-scrolling platformer designed for a young child (age ~4), where the player controls a customizable superhero character navigating fun, themed levels (forest, sky, candy land, etc.).

The focus is:
- Simple controls
- Rewarding feedback
- Low frustration
- Expandability over time

---

## 2. Core Concept

- Side-scrolling platformer (inspired by Mario-style gameplay)
- Player runs, jumps, collects items, and reaches the end of the level
- Bright, friendly visuals
- Optional “superpowers” (float, double jump, etc.)

---

## 3. Target Experience

### Design Goals
- Easy to learn (playable in <30 seconds)
- Positive reinforcement (sounds, animations, rewards)
- No punishing mechanics (minimal or no death early on)
- Engaging for both child and parent (you 😄)

---

## 4. Core Features (MVP)

### Player Mechanics
- Move left/right
- Jump (gravity-based physics)
- Collision with platforms
- Simple animation (idle, run, jump)

### World
- Side-scrolling camera
- Platforms (ground + floating)
- One level

### Collectibles
- Stars / hearts / animals
- Increment score when collected

### Goal
- Reach end-of-level flag or object

### UI
- Score display
- High score display

### Persistence
- Save/load high score to local file (JSON)

---

## 5. Post-MVP Features (Expansion Roadmap)

### Gameplay Expansion
- Multiple levels (themes: forest, space, underwater)
- Enemies (slow-moving, non-threatening at first)
- Power-ups:
  - Double jump
  - Glide/fly
  - Temporary invincibility

### Personalization
- Character customization (colors, outfit)
- Name input

### Interaction
- Sound effects and music
- Voice lines or fun reactions

### Progression
- Level unlock system
- Save progress

### Polish
- Particle effects
- Better animations
- Smooth transitions

---

## 6. Game Logic

### Game Loop
Handled by Bevy ECS:
- Input system → movement system → physics → collision → rendering

### Movement
- Horizontal velocity based on input
- Jump applies upward velocity
- Gravity constantly pulls downward

### Collision
- AABB (axis-aligned bounding box)
- Player vs platforms
- Player vs collectibles

### Scoring
- +1 (or more) per collectible
- Score stored in memory during run
- On level complete → compare to high score

### High Score
- Stored in JSON file:
```json
{ "high_score": 42 }
7. Tech Stack
Language
Rust
Game Engine
Bevy (ECS-based, modern Rust engine)
Key Crates
bevy → rendering, ECS, input, physics basics
serde → serialization
serde_json → saving/loading high score
(optional later) bevy_kira_audio → audio
Assets
Placeholder sprites initially:
Colored rectangles
Basic shapes
File Storage
Local JSON file:
save.json
8. Art Strategy
Phase 1 (MVP)
Use placeholders:
Squares for player
Rectangles for platforms
Circles for collectibles
Phase 2
Replace with free assets from:
itch.io
OpenGameArt
Phase 3
Custom assets (optional):
Aseprite or similar tools
9. Architecture (Bevy ECS)
Entities
Player
Platform
Collectible
Camera
Components
Position (Transform)
Velocity
Collider
PlayerTag
CollectibleTag
Score
Systems
Input system
Movement system
Gravity system
Collision system
Score system
Rendering handled by Bevy
10. MVP Build Plan (Step-by-Step)
Step 1: Setup Project
Install Rust
cargo new superhero_platformer
Add Bevy dependency
Step 2: Basic Window
Launch Bevy app
Display empty screen
Step 3: Spawn Player
Add a simple square sprite
Position it on screen
Step 4: Movement
Add left/right movement via keyboard
Add jump mechanic
Step 5: Gravity
Apply constant downward force
Prevent infinite jumping
Step 6: Platforms
Add static ground
Add a few floating platforms
Step 7: Collision
Player lands on platforms
Prevent falling through
Step 8: Camera
Follow player horizontally
Step 9: Collectibles
Spawn collectible objects
Detect collision → increase score
Step 10: UI
Display score on screen
Step 11: High Score Save
On game end:
Save score to JSON
On start:
Load high score
Step 12: End Condition
Add goal object (flag)
Trigger level completion
11. Controls
Left Arrow / A → Move left
Right Arrow / D → Move right
Space → Jump

(Controller support can be added later)

12. Future Enhancements
Touchscreen support (tablet-friendly)
Controller input
Procedural level generation
Co-op mode (parent + child)
Educational elements (letters, numbers, animals)
13. Development Philosophy
Build fast, iterate often
Keep everything playable at all times
Prioritize fun over correctness
Replace placeholders only after gameplay feels good
14. Stretch Ideas (Optional but Cool)
Let your child design levels (simple editor)
Record voice clips for in-game reactions
Add a “photo mode” or replay system
Dynamic difficulty (auto-adjust based on performance)
15. Summary

Start simple:

One character
One level
One goal

Then expand:

More mechanics
More personality
More fun