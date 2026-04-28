import numpy as np
from PIL import Image
from collections import deque
import sys


def remove_background(input_file, output_file, tolerance=25, grow_tolerance=60, grow_passes=3):
    img = Image.open(input_file).convert("RGBA")
    arr = np.array(img, dtype=np.uint8)
    h, w = arr.shape[:2]

    bg = arr[0, 0, :3].astype(int)
    print(f"Background color: rgb({bg[0]}, {bg[1]}, {bg[2]})")

    # Step 1: Flood fill from edges to remove solid background
    mask = np.zeros((h, w), dtype=bool)
    visited = np.zeros((h, w), dtype=bool)
    queue = deque()

    for x in range(w):
        queue.append((0, x))
        queue.append((h - 1, x))
    for y in range(h):
        queue.append((y, 0))
        queue.append((y, w - 1))

    while queue:
        r, c = queue.popleft()
        if visited[r, c]:
            continue
        visited[r, c] = True

        if np.abs(arr[r, c, :3].astype(int) - bg).max() > tolerance:
            continue

        mask[r, c] = True

        for dr, dc in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            nr, nc = r + dr, c + dc
            if 0 <= nr < h and 0 <= nc < w and not visited[nr, nc]:
                queue.append((nr, nc))

    # Step 2: Grow transparency outward — catches anti-aliased edge pixels
    bg_arr = bg.reshape(1, 1, 3)
    for _ in range(grow_passes):
        adj = np.zeros((h, w), dtype=bool)
        adj[1:, :]   |= mask[:-1, :]
        adj[:-1, :]  |= mask[1:, :]
        adj[:, 1:]   |= mask[:, :-1]
        adj[:, :-1]  |= mask[:, 1:]
        adj[1:, 1:]  |= mask[:-1, :-1]
        adj[1:, :-1] |= mask[:-1, 1:]
        adj[:-1, 1:] |= mask[1:, :-1]
        adj[:-1, :-1]|= mask[1:, 1:]

        candidates = adj & ~mask
        close_to_bg = np.abs(arr[:, :, :3].astype(int) - bg_arr).max(axis=2) <= grow_tolerance
        mask |= candidates & close_to_bg

    arr[:, :, 3] = np.where(mask, 0, arr[:, :, 3])

    # Step 3: Despill background color fringe from remaining edge pixels.
    # Find which channel the bg has least of — that's the clean reference.
    # Despill the two channels that the bg has most of.
    bg_r, bg_g, bg_b = int(bg[0]), int(bg[1]), int(bg[2])
    bg_max = max(bg_r, bg_g, bg_b)
    bg_min_ch = min(('r', bg_r), ('g', bg_g), ('b', bg_b), key=lambda x: x[1])[0]

    if bg_max > 150 and {'r': bg_r, 'g': bg_g, 'b': bg_b}[bg_min_ch] < 80:
        adj = np.zeros((h, w), dtype=bool)
        adj[1:, :]  |= mask[:-1, :]
        adj[:-1, :] |= mask[1:, :]
        adj[:, 1:]  |= mask[:, :-1]
        adj[:, :-1] |= mask[:, 1:]
        edge = adj & ~mask

        r = arr[:, :, 0].astype(float)
        g = arr[:, :, 1].astype(float)
        b = arr[:, :, 2].astype(float)

        ref = {'r': r, 'g': g, 'b': b}[bg_min_ch]

        if bg_min_ch == 'r':    # cyan/blue-green bg: despill G and B
            s1, s2 = np.maximum(0.0, g - ref), np.maximum(0.0, b - ref)
            correction = np.minimum(s1, s2) * 0.8
            apply = edge & (s1 > 15) & (s2 > 15)
            g[apply] = np.maximum(0.0, g[apply] - correction[apply])
            b[apply] = np.maximum(0.0, b[apply] - correction[apply])
            arr[:, :, 1] = g.clip(0, 255).astype(np.uint8)
            arr[:, :, 2] = b.clip(0, 255).astype(np.uint8)
        elif bg_min_ch == 'g':  # magenta/red-blue bg: despill R and B
            s1, s2 = np.maximum(0.0, r - ref), np.maximum(0.0, b - ref)
            correction = np.minimum(s1, s2) * 0.8
            apply = edge & (s1 > 15) & (s2 > 15)
            r[apply] = np.maximum(0.0, r[apply] - correction[apply])
            b[apply] = np.maximum(0.0, b[apply] - correction[apply])
            arr[:, :, 0] = r.clip(0, 255).astype(np.uint8)
            arr[:, :, 2] = b.clip(0, 255).astype(np.uint8)
        else:                   # yellow/red-green bg: despill R and G
            s1, s2 = np.maximum(0.0, r - ref), np.maximum(0.0, g - ref)
            correction = np.minimum(s1, s2) * 0.8
            apply = edge & (s1 > 15) & (s2 > 15)
            r[apply] = np.maximum(0.0, r[apply] - correction[apply])
            g[apply] = np.maximum(0.0, g[apply] - correction[apply])
            arr[:, :, 0] = r.clip(0, 255).astype(np.uint8)
            arr[:, :, 1] = g.clip(0, 255).astype(np.uint8)

    Image.fromarray(arr).save(output_file)
    print(f"Saved -> {output_file}")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python remove_bg.py input.png [output.png]")
        sys.exit(1)

    input_file = sys.argv[1]
    output_file = sys.argv[2] if len(sys.argv) >= 3 else input_file.rsplit(".", 1)[0] + "_transparent.png"

    remove_background(input_file, output_file)
