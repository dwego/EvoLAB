# `graphics/` — Visualization and instrumentation

**Implemented**: camera, light and spring gizmo. **Not in the inspected remote revision**: committed grid, switchable cameras, scientific HUD.

## Files

- `scene.rs`: camera and point light.
- `camera.rs`: `FollowCamera` reads creature's mean position and uses exponential interpolation toward an offset viewpoint.
- `debug.rs`: draws spring and phase-based grip markers; also contains a not-yet-registered `trace_motion` diagnostic.
- `mod.rs`: schedules rendering and camera tracking before transform propagation.

The camera uses a smoothing coefficient

$$
\alpha=1-e^{-4\Delta t}.
$$

This reduces frame-rate dependency of camera translation interpolation.

## Known visual mismatch

The **red/green indicators are computed from cosine phase**, while the working neural controller produces **two continuous grip commands**. Until changed, those indicators **must not** be presented as neural telemetry.

## Next

Add meters-aligned grid, fixed/top/orbit modes, camera-relative labels, energy meters, brain output values, fitness plots and playback controls. Visuals must observe simulation state, not change it.
