# Servo Paint Spike Branch Plan

Branch: spike/servo-paint-pipeline

## Goal

Replace snapshot-bridge visual rendering with a true Servo-based DOM/CSS/layout/paint pipeline while keeping a safe fallback path during migration.

## What Is Implemented In This Spike Start

- Introduced visual backend selection via `CATISEN_VISUAL_ENGINE`.
  - `snapshot` (default)
  - `servo-spike`
- Added staged Servo spike renderer path that writes a per-frame manifest:
  - `target/servo_spike/manifests/frame-<timestamp>.json`
- Added marker protocol for spike output:
  - `__VISUAL_SPIKE_MANIFEST__::...`
- Added UI handling in Visual mode to display manifest status cleanly.
- Kept temporary snapshot fallback (toggle with `CATISEN_SERVO_SPIKE_SNAPSHOT_FALLBACK`).

## Next Spike Steps

1. DOM parse stage
- Hook Servo HTML parser and persist parse diagnostics in manifest.

2. Style resolve stage
- Build stylesheet pipeline and computed-style snapshots for selected nodes.

3. Layout stage
- Add layout tree generation and viewport metrics to manifest.

4. Compositing stage
- Produce an actual pixel buffer/frame output from Servo pipeline.

5. egui presentation stage
- Upload rendered frame texture directly into egui without external browser screenshot.

6. Performance instrumentation
- Record stage timings (`dom_ms`, `style_ms`, `layout_ms`, `paint_ms`, `composite_ms`) per frame.

## Validation Criteria

- Visual mode renders DOM/CSS content without relying on external browser screenshot.
- Snapshot fallback is no longer needed for normal pages.
- FPS and memory metrics improve predictably across 90/120/144/150 targets.
