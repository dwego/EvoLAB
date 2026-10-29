# \`brain/\` — Neural control

**Implemented**: a pure Rust MLP and its Bevy ECS controller. **Not implemented**: learning, mutation, serialization and recurrent memory.

## Source files

- \`network.rs\`: \`NeuralNetwork\` stores a 4→6→3 topology with 51 weights/biases and computes \`forward([f32;4]) -> [f32;3]\`.
- \`controller.rs\`: \`NeuralController\` stores network and \`MotorCommands\`; \`update_brains\` samples \`Position\`, \`LinearVelocity\`, computes normalized observations and commands.
- \`mod.rs\`: \`BrainPlugin\` schedules \`update_brains\` in \`FixedUpdate\`.

## Contract with physics

Inputs: normalized spring displacement, axial relative speed, phase sine, phase cosine. Outputs: motor \([-1,1]\), left and right grip \([0,1]\). Physics consumes **commands**, not the raw neural matrices.

\`baseline()\` is manually hand-wired to mimic periodic gait. It is a diagnostic reference brain, **not a trained policy**.

## Testing and next tasks

One current unit test checks that opposite phases reverse grip preference. Next: add output bounds, finite-value validation, seeded initialization and mutation tests. Then expose weights through a serializable genotype. For full derivations see [Neural controller](../../docs/neural-controller.md).

![Network](../../assets/brain.svg)
