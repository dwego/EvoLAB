# \`creatures/\` — Embodiment and genome

**Implemented**: a two-body crawler. **Proposed**: arbitrary body graphs, morphology genotypes, reproduction and energy components.

## Files and components

- \`mod.rs\`: \`Creature { masses: [Entity;2], genome, stiffness, damping, elapsed }\` and \`CreaturePlugin\`.
- \`genome.rs\`: \`Genome { rest_length, amplitude, frequency, phase_offset }\`, default values 1.2 m, 0.35 m, 1.2 Hz, 0 radians.
- \`spawn.rs\`: creates two spheres and one logical organism with \`NeuralController::default()\`.

## Geometry

Each sphere: radius 0.25 m, mass 1 kg, positions \((-0.6,0.27,0)\) and \((+0.6,0.27,0)\); both dynamic with rotation locked. Their collider friction is set to zero for the current force-based grip prototype.

The component points to two **ECS entities**. It does not duplicate physics positions. This is intentional: Avian owns the actual rigid-body state.

## Evolution implications

The current \`Genome\` parameterizes an existing body and muscle; it does not yet encode number of segments, bones, joints, sensors or neural weights. Evolution will require a distinct reproducible and serializable body/brain genotype schema, with explicit constraints to prevent invalid organisms.

![Mechanics](../../assets/physics.svg)

Full specification: [Physics](../../docs/physics.md) · [Future ecology](../../docs/artificial-life.md).
