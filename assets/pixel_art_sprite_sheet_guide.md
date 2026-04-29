# Pixel Art Sprite Sheet Guide

Use this prompt **plus a reference photo** when generating sprite sheets with an LLM image model.

---

## Prompt Template

> Using the attached reference image, create a game-ready pixel art sprite sheet of a young girl superhero character.

---

## Style

- Clean 2D pixel art
- Crisp hard edges — **no anti-aliasing, no blur, no painterly texture**
- Limited retro palette (16–32 colors)
- Consistent proportions
- Polished indie-game quality

---

## Camera

- Side-view 2D platformer angle
- Full body visible
- Fixed camera distance
- Identical framing in every frame

---

## Sprite Sheet Layout

| Property | Value |
|---|---|
| Grid | 5 columns × 5 rows |
| Total frames | 25 |
| Frame size | 128 × 128 px |
| Final image size | 640 × 640 px |
| Padding / borders / grid lines | **None** |

---

## Animation Rows

| Row | Animation | Frames |
|---|---|---|
| 1 | Idle | 5 |
| 2 | Running | 5 |
| 3 | Jumping | 5 |
| 4 | Flying | 5 |
| 5 | Celebrating | 5 |

### Animation Notes

**Idle** — subtle breathing, cape flutter, slight hair movement

**Running** — strong readable run cycle, dynamic cape motion

**Jumping** (frame order):
1. Crouch
2. Launch
3. Ascent
4. Apex
5. Descent

**Flying** (frame order):
1. Hover
2. Accelerate
3. Glide
4. Bank
5. Hover

**Celebrating** — happy victory pose, energetic movement

---

## Alignment Rules *(critical)*

- Torso center **locked at x=64** in every frame
- Standing feet baseline aligned consistently in every grounded frame
- No horizontal drift
- No scale change
- No camera rotation
- Identical body proportions in all frames

---

## Safe Bounds

Keep the character inside an **8 px inner margin** on all sides of each frame — no clipping.

---

## Character Consistency

Maintain identical across all frames:

- Face & hairstyle
- Costume & colors
- Cape length
- Boot & glove proportions

---

## Background

- **Pure magenta `#FF00FF`** — solid, uniform
- No gradients
- No texture
- No shadows on background

---

## Output

Single PNG sprite sheet only.

---

## Post-Processing Workflow

> The LLM will likely return an image at the wrong size. Follow these steps to fix it.

### 1. Resize to exact dimensions

```bash
convert input_image.png -filter point -resize 640x640! output_image.png
```

Forces the image to 640×640 (a 5×5 grid of 128×128 frames) using nearest-neighbour scaling to preserve pixel art crispness.

### 2. Remove the magenta background

```bash
python scripts/remove_bg.py input_image.png output_image.png
```

### 3. Move to the assets folder

```bash
mv output_image.png assets/<character_name>/
```

For example, a character named `bridget` goes to `assets/bridget/`.
