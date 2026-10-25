# Performance investigation — v0.6.0

Measured optimized native Windows builds on Intel Iris Plus Graphics, fullscreen, 960 frames walking out and back along the 20-level trail. The test writes to an isolated save and excludes screenshot exports. Raw JSON reports are retained with the release review files.

| Measurement | Before | After |
|---|---:|---:|
| Frame median | 17.12 ms | 16.84 ms |
| Frame p95 | 240.20 ms | 66.33 ms |
| Frame p99 | 260.44 ms | 68.84 ms |
| Frames over 50 ms | 292 / 960 | 133 / 960 |
| Warm return frames over 50 ms | — | 64 / 480 |
| Navigation p95 | 13.27 ms | 0.022 ms |
| Save call p95 | 13.26 ms | 0.017 ms |

The maximum was 443 ms during initial setup. Periodic rendering stalls remain on this integrated GPU, including on the return trip. The 95th-percentile frame time is about 72% lower, but this does not eliminate every stutter. This is a measured improvement, not a promise of uninterrupted 60 FPS on every machine. The original run used the real mouse hover, while the later run fixes hover over the travel arrow for reproducibility.

## Findings and changes

Synchronous atomic save writes blocked movement handlers. Saves now run on a coalescing worker; orderly shutdown flushes pending progress. Level sessions are prepared once and cloned when starting/restarting, avoiding repeated validation and route searches during navigation.

A blank-window control was steady at a 17 ms p95. The actual scene generated many geometry batches and texture switches. Original piece artwork is now baked once into a shared 3x sprite atlas; sprite and text draws are grouped by layer. Fonts have separate caches per raster size, preventing glyph corruption when a large shared atlas grows; this retains readable text at some batching cost. Rounded panels use a single non-overlapping mesh and curves use enough segments for subpixel edges. Font glyphs warm up on display-size changes, with glyph preparation before queued text rendering.

The existing meadow resolution and crisp native text remain. The universe artwork is 1586 × 992, loaded once. Four-sample antialiasing remains enabled. Testing one-sample antialiasing did not resolve the stalls. Releasing an already-free cursor cost less than 0.001 ms and was not the cause. No live opponent search was responsible: update p95 was 0.003 ms in the final trail run.

Reproduce with `--performance-test --performance-report PATH --performance-frames 960`. Use an optimized build. Diagnostic flags `--performance-clear-only`, `--performance-raw-art`, `--performance-msaa1`, and `--performance-windowed` support controls. Screenshot export and offline level-authoring searches are deliberately outside frame timings.


## v0.7 follow-up: sharp text with fewer texture switches

The per-raster font workaround kept text readable but added texture switches. Inspection of Macroquad 0.4.14's bundled default shader found `varying lowp vec2 uv`. Large shared font atlases showed visibly damaged letters on this Intel GPU. A dedicated text material now uses high-precision UVs, preserving the same alpha blend. Glyph preparation happens before text submission, and atlas resets on display-scale changes bound resize cache growth. Native Swedish/English settings and 4K screenshots were visually rechecked.

A final 960-frame optimized, symbol-retaining release run on the same Intel Iris Plus GPU measured:

| Measurement | Original v0.5 baseline | Final v0.7 |
|---|---:|---:|
| Frame median | 17.12 ms | 16.24 ms |
| Frame p95 | 240.20 ms | 38.11 ms |
| Frame p99 | 260.44 ms | 47.34 ms |
| Frames over 50 ms | 292 / 960 | 9 / 960 |
| Warm return frames over 50 ms | — | 1 / 480 |
| Warm return p95 | 195.97 ms | 36.92 ms |
| Navigation p95 | 13.27 ms | 0.293 ms |

The p95 reduction is about 84%. Startup/fullscreen/cache preparation still had a 396 ms maximum, and the warm return had one 58 ms frame. There is no claim of a perfectly steady 60 FPS. The first baseline used the real mouse; the repeatable test fixes hover over navigation arrows. These are observations on this machine, not controlled cross-hardware benchmarks. The final profile used real saves in an isolated temporary directory and excluded PNG export.
