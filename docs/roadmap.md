# Roadmap — from the crawler to artificial life

This is a planning document, **not** a list of delivered features.

| Stage | Status | Scope | Exit criterion |
|---|---|---|---|
| 0.1 — Genesis | **implemented prototype** | Bevy world, Avian dynamics, spring, two spheres | two-body actuation observable |
| 0.2 — Neural controller | **implemented baseline** | 4→6→3 MLP and sensor/motor bridge | hand-wired controller drives the crawler |
| 0.2.x — Lab instrumentation | **partially prototyped locally** | grid, camera modes, world beyond finite floor, telemetry | motion can be measured and replayed |
| 0.3 — Episode runner | **planned** | deterministic resettable trial harness | reproducible 10-second evaluations |
| 0.4 — Neuroevolution | **planned** | seeded population, selection, mutation | compared to random/hand-wired baselines |
| 0.5 — Energy economics | **planned** | cost for basal metabolism and motors | conserved, auditable energy accounting |
| 0.6 — Resource ecology | **planned** | food patches, sensing, consumption | organisms must seek energy |
| 0.7 — Reproduction | **planned** | inheritance, mutation, offspring, death | lineage tracking without corrupting ECS |
| 0.8 — New morphologies | **planned** | joints, legs, body/genome graph | viable bodies from genotype |
| 0.9 — Ecological diversity | **planned** | niches, experiments, multiple lineages | measurable behavior/lineage diversity |
| 1.0 — Research platform | **aspirational** | repeatable experiments, dataset, replay, docs | external contributor reproduces findings |

## Near-term engineering work

1. Preserve the working crawler as a regression benchmark.
2. Fix visual markers to show actual `MotorCommands`.
3. Build a clean telemetry overlay in world coordinates.
4. Add camera controls and an absolute-reference spatial grid.
5. Decide on a robust physical-floor strategy.
6. Separate rendering from the evaluation harness.
7. Add unit and integration tests for forces and network boundaries.
8. Introduce an experiment manifest and snapshots.
9. Compare random and hand-wired policies.
10. Add evolutionary mutation only after repeatable episodes exist.

## Technical debt

- Sphere-body grip is a force approximation, not true static contact.
- Physical friction is zero on the sphere colliders in the functional locomotion prototype.
- The debug markers still encode a phase heuristic, not neural control outputs.
- A `trace_motion` function exists but is not registered in the GitHub state inspected.
- `rand` is a declared dependency but no evolutionary loop exists.
- Only one neural test currently exists.
- A finite ground plane is present in the documented remote version.
- No policy serialization, world reset, fitness tracking, energy, food or offspring.
- No open-source license file yet.

## Principle

Increase **scientific validity before feature count**. Every major system must have unit tests, failure modes, and at least one experiment distinguishing expected behavior from an implementation artifact.
