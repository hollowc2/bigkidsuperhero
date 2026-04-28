from PIL import Image
from collections import deque
import os
import json
import sys
import argparse


# -----------------------------
# STEP 1: EDGE FLOOD FILL BG REMOVAL
# -----------------------------

def color_close(c1, c2, tol=20):
    return (
        abs(c1[0] - c2[0]) <= tol and
        abs(c1[1] - c2[1]) <= tol and
        abs(c1[2] - c2[2]) <= tol
    )


def remove_bg(img, tolerance=20, bg_color=None):
    img = img.convert("RGBA")
    pixels = img.load()
    w, h = img.size

    corner = pixels[0, 0]
    if bg_color:
        bg = tuple(bg_color)
    elif corner[3] > 0:
        bg = corner[:3]
    else:
        # Corner already transparent — scan edges for first opaque pixel
        bg = None
        for x in range(w):
            for y in [0, h - 1]:
                if pixels[x, y][3] > 0:
                    bg = pixels[x, y][:3]
                    break
            if bg:
                break
        if not bg:
            for y in range(h):
                for x in [0, w - 1]:
                    if pixels[x, y][3] > 0:
                        bg = pixels[x, y][:3]
                        break
                if bg:
                    break
        if not bg:
            bg = (0, 0, 0)  # fallback, flood fill becomes no-op

    q = deque()
    visited = set()

    # add edges
    for x in range(w):
        q.append((x, 0))
        q.append((x, h - 1))

    for y in range(h):
        q.append((0, y))
        q.append((w - 1, y))

    while q:
        x, y = q.popleft()

        if (x, y) in visited:
            continue
        visited.add((x, y))

        if not color_close(pixels[x, y][:3], bg, tolerance):
            continue

        pixels[x, y] = (0, 0, 0, 0)

        for nx, ny in [(x+1,y),(x-1,y),(x,y+1),(x,y-1)]:
            if 0 <= nx < w and 0 <= ny < h:
                q.append((nx, ny))

    # Second pass: remove interior enclosed pixels that still match bg color
    for y in range(h):
        for x in range(w):
            if pixels[x, y][3] > 0 and color_close(pixels[x, y][:3], bg, tolerance):
                pixels[x, y] = (0, 0, 0, 0)

    return img


# -----------------------------
# STEP 2: SPLIT SPRITE SHEET
# -----------------------------

def slice_sheet(img, cols=5, rows=5, size=128, out_dir="output_frames"):
    os.makedirs(out_dir, exist_ok=True)

    frames = []
    index = 0

    meta = {}

    for r in range(rows):
        for c in range(cols):
            left = c * size
            top = r * size
            right = left + size
            bottom = top + size

            frame = img.crop((left, top, right, bottom))

            name = f"frame_{index:02d}.png"
            path = os.path.join(out_dir, name)

            frame.save(path)

            frames.append(path)
            meta[name] = {
                "row": r,
                "col": c,
                "x": left,
                "y": top
            }

            index += 1

    return frames, meta


# -----------------------------
# STEP 3: PIPELINE
# -----------------------------

def process(input_file, bg_color=None, tolerance=20):
    base = os.path.splitext(input_file)[0]

    print("[1/3] Loading image...")
    img = Image.open(input_file)

    print("[2/3] Removing background...")
    clean = remove_bg(img, tolerance=tolerance, bg_color=bg_color)

    clean_path = base + "_transparent.png"
    clean.save(clean_path)
    print("Saved:", clean_path)

    print("[3/3] Slicing sprite sheet...")
    frames, meta = slice_sheet(clean)

    meta_path = os.path.join("output_frames", "meta.json")
    with open(meta_path, "w") as f:
        json.dump(meta, f, indent=2)

    print("Saved frames:", len(frames))
    print("Saved metadata:", meta_path)

    print("\nDONE.")


# -----------------------------
# RUN
# -----------------------------

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("input", help="Sprite sheet PNG")
    parser.add_argument("--bg", help="Background color as R,G,B (e.g. 0,255,255)", default=None)
    parser.add_argument("--tol", type=int, default=20, help="Color tolerance (default 20)")
    args = parser.parse_args()

    bg_color = None
    if args.bg:
        bg_color = tuple(int(x) for x in args.bg.split(","))

    process(args.input, bg_color=bg_color, tolerance=args.tol)
