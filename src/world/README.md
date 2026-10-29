# \`world/\` — The physical environment

**Implemented on the inspected GitHub revision:** a static 200 × 0.2 × 40 m ground cuboid centered at \((0,-0.1,0)\), with its top at \(Y=0\). The world plugin registers the setup system at startup.

The floor includes mesh, material, \`RigidBody::Static\`, matching \`Collider::cuboid\`, and \`Friction::new(1.0)\`.

## Why this module exists

World state should describe terrain, physical obstacles, resource distribution and environmental conditions. The world should never depend on a specific neural controller. Eventually an experiment config should deterministically generate a scenario and reset it.

## Known limitation

The committed floor is finite. A follow camera may continue moving after the creature leaves this ground, but gravity/collision support will not. A later local experiment with infinite physical ground has been discussed, but is not represented as a committed feature here.

## Future environments (planned)

- Infinite or chunked terrain with world-space reference grid.
- Resource patches and energy transfer.
- Obstacle courses with configurable layouts.
- Temperature/light/chemical gradients, if later justified.
- Multi-organism collision/competition testbeds.
- Reproducible world generation seeded per experiment.

![Ecology concept](../../assets/ecosystem.svg)

See [Artificial-life design](../../docs/artificial-life.md).
