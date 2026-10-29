# Experimental design and reproducibility

EvoLab is meant to enable reproducible experiments, not just convincing animations.

## Research log: what has actually been observed

Exploratory observations from development:

1. When motor amplitude is zero, the two-mass spring settles rather than performing continuous visible actuation.
2. Under high/low friction experiments, debug colors alternated but displacement did not improve meaningfully; the spring length barely changed.
3. When friction was set to zero, the spring distance varied substantially (sampled range roughly 0.79–1.63 m in one run) while center X stayed near 0.
4. With the later explicit, contact-aware, asymmetric velocity-resistance force, the two-mass system visibly advanced.
5. A hand-wired neural network was then connected to the motor and grip outputs and the creature walked out of a fixed camera view.

**These are qualitative prototype observations**, not a peer-reviewed benchmark. The zero-friction data come from manually collected logs; they are not committed as a test dataset in the repo.

## Control cases

| Control | What it isolates |
|---|---|
| No motor actuation | passive spring behavior |
| No grip | internal force conservation / lack of COM acceleration |
| Symmetric grip | effect of asymmetric contact |
| Hand-wired brain | current functional reference |
| All-zero network | neutral baseline |
| Random network | chance performance |
| Randomized seeds | robustness |
| Half timestep | numerical convergence |

Do not make fitness claims until a candidate is evaluated against at least baseline and random controls.

## Suggested metrics

Define position and time in consistent SI units and record at fixed cadence.

- \(\Delta x\): signed center-of-mass displacement.
- \(\bar v_x = \Delta x/T\): average forward speed.
- \(E_{\rm actuator}\): work/consumption proxy—requires explicit modeling.
- \(\eta\): locomotion efficiency (once energy is implemented).
- \(t_{\rm alive}\): survival duration in resource scenarios.
- \(N_{\rm viable\,offspring}\): evolutionary persistence.
- Stability: invalid contacts, NaNs, extreme forces and unstable dynamics.
- Diversity: pairwise neural or behavioral distances.

## Episode manifest (proposed)

~~~yaml
experiment_id: crawler-baseline-001
commit: "<git-sha>"
physics:
  engine: avian3d-0.8
  fixed_step_seconds: "<actual configured value>"
world:
  gravity: "<read from actual world resource>"
  terrain: finite-ground
subject:
  body: two-sphere
  brain: hand-wired-baseline
evaluation:
  episode_seconds: 10
  seeds: [42, 43, 44]
  selection: none
outputs:
  measurements: results.csv
  replay: best.json
~~~

Do not fill unknown defaults in a public result by guessing. Capture actual runtime values.

## Recommended statistical reporting

For multiple seeds, show central tendency and variation. Report sample size and avoid over-interpreting a single best run. Maintain baselines across versions with saved genomes and configs. If comparing algorithms, use identical scenario panels whenever possible.

## Physical sanity checks

- Mechanical spring force sum should be zero for every pair.
- Passive damping should remove relative kinetic energy, excluding integration artifacts.
- The center of mass of an isolated two-body spring should maintain constant velocity.
- Grip force must oppose horizontal velocity.
- Physics state should remain finite and bounded.
- Genome bounds should prevent invalid motor targets and nonphysical springs.

## Reproduction checklist

1. Record commit, compiler versions and platform.
2. Record all random seeds and episode parameters.
3. Start from a clean, specified initial world.
4. Run a fixed number of physics steps independent of FPS.
5. Save data **before** rendering or UI aggregation.
6. Repeat with multiple seeds.
7. Compare with controls and report failures.
8. Archive exact code and trained parameters used.

## Visualization integrity

The camera follows the organism and therefore can make a stationary subject appear different from a moving one; use world-space coordinates and fixed grid markers as references. The current gizmo colors depict the **old phase rule**, not the actual neural grip outputs; they should be corrected before presenting them as controller telemetry.
