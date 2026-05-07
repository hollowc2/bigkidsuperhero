#!/usr/bin/env python3
"""
Horizontally centers each sprite frame by its TORSO center of mass,
not its full bounding box. This ignores the cape, which blows to
different sides each frame and would throw off a bounding-box approach.

Strategy: look only at the top 55% of each cell (head + torso, above
where the cape typically flows) and use the weighted center of mass of
all opaque pixels in that band.
"""

from PIL import Image
import numpy as np
import os
import shutil
from collections import deque

ASSETS = os.path.join(os.path.dirname(__file__), "..", "assets")
ALPHA_THRESHOLD = 10
TORSO_BAND = 0.55   # use top 55% of cell height to find body center

SHEETS = [
    # (filename, cell_w, cell_h, cols, rows)
    # These are the 5x5 player sheets used by the game.
    ("bridget/bridget_cartoon_sprite.png", 128, 128, 5, 5),
    ("calvin/calvin_sprite.png", 128, 128, 5, 5),
]


def torso_cx(cell: np.ndarray, alpha_thresh: int, band: float) -> float | None:
    """Return weighted horizontal center of mass of the torso band, or None."""
    band_h = max(1, int(cell.shape[0] * band))
    region = cell[:band_h, :, 3].astype(np.float32)
    region[region < alpha_thresh] = 0.0
    total = region.sum()
    if total == 0:
        return None
    col_indices = np.arange(cell.shape[1], dtype=np.float32)
    return float((region.sum(axis=0) * col_indices).sum() / total)


def remove_top_artifacts(cell: np.ndarray, alpha_thresh: int = ALPHA_THRESHOLD,
                         max_area: int = 180, max_top: int = 8) -> bool:
    """Remove tiny detached components near the top of a frame."""
    alpha = cell[:, :, 3]
    h, w = alpha.shape
    seen = np.zeros((h, w), dtype=bool)
    changed = False

    for y in range(h):
        for x in range(w):
            if seen[y, x] or alpha[y, x] <= alpha_thresh:
                continue

            q = deque([(x, y)])
            seen[y, x] = True
            pts = []

            while q:
                cx, cy = q.popleft()
                pts.append((cx, cy))
                for nx, ny in ((cx + 1, cy), (cx - 1, cy), (cx, cy + 1), (cx, cy - 1)):
                    if 0 <= nx < w and 0 <= ny < h and not seen[ny, nx] and alpha[ny, nx] > alpha_thresh:
                        seen[ny, nx] = True
                        q.append((nx, ny))

            if len(pts) <= max_area:
                ys = [py for _, py in pts]
                if min(ys) <= max_top:
                    for px, py in pts:
                        cell[py, px, :] = 0
                    changed = True

    return changed


def center_sheet(path, cell_w, cell_h, cols, rows):
    img = Image.open(path).convert("RGBA")
    data = np.array(img, dtype=np.uint8)

    changed = False
    for row in range(rows):
        for col in range(cols):
            x0, y0 = col * cell_w, row * cell_h
            x1, y1 = x0 + cell_w, y0 + cell_h
            cell = data[y0:y1, x0:x1].copy()

            cx = torso_cx(cell, ALPHA_THRESHOLD, TORSO_BAND)
            if cx is None:
                continue

            target = cell_w / 2.0
            shift = round(target - cx)
            shifted = np.zeros_like(cell)
            if shift > 0:
                w = cell_w - shift
                shifted[:, shift:shift + w] = cell[:, :w]
            elif shift < 0:
                s = -shift
                w = cell_w - s
                shifted[:, :w] = cell[:, s:s + w]
            else:
                shifted[:] = cell

            artifact_removed = False
            if row in (1, 2, 3):
                artifact_removed = remove_top_artifacts(shifted)

            if shift != 0 or artifact_removed:
                data[y0:y1, x0:x1] = shifted
                changed = True
                note = " +clean" if artifact_removed else ""
                print(f"  [{row},{col}] torso_cx={cx:.1f} → {target:.1f}, shift={shift:+d}px{note}")

    if not changed:
        print("  All frames already centered — no changes made.")
        return

    backup = path.replace(".png", "_backup_prealign.png")
    if not os.path.exists(backup):
        shutil.copy2(path, backup)
        print(f"  Backup → {os.path.basename(backup)}")

    Image.fromarray(data).save(path)
    print(f"  Saved  → {os.path.basename(path)}")


if __name__ == "__main__":
    for fname, cw, ch, cols, rows in SHEETS:
        path = os.path.join(ASSETS, fname)
        if not os.path.exists(path):
            print(f"Skipping {fname} (not found)")
            continue
        print(f"\n{fname}  ({cols}×{rows} cells, {cw}×{ch}px each)")
        center_sheet(path, cw, ch, cols, rows)

    print("\nDone.")
