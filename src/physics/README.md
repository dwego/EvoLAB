# `physics/` — Forces and actuators

**Implemented**: Hooke + damping and a temporary contact-aware resistive grip. Avian integrates dynamics and resolves contacts.

## Files

- `mod.rs`: registers spring then grip force systems as a fixed ordered chain in `FixedPostUpdate`, between Avian preparation and step.
- `springs.rs`: computes real separation, neural target length, axial velocity, forces $\mathbf{F}_a=-\mathbf{F}_b$, and increments the organism's elapsed simulated time.
- `grip.rs`: applies horizontal velocity-opposing force based on each neural grip signal if the sphere is near the ground.

## Physical model

$$
\mathbf F_a=\left[k(\ell-\ell_t)+c(\mathbf v_b-\mathbf v_a)\cdot\hat{\mathbf d}\right]\hat{\mathbf d},\quad
\mathbf F_b=-\mathbf F_a.
$$

$$
\ell_t=\max(0.3,\ell_0+A u_m)
$$

Contact-aware grip:

$$
\mathbf F_{\rm grip}=
\mathrm{clampMagnitude}(-45 g\mathbf v_{xz},35g).
$$

**Caution:** this is *not* true static friction. The two sphere colliders currently use `Friction::new(0.0)`. A later grounded leg mechanism should use validated contact/joint physics.

## Testing

Add tests for force antisymmetry, damping sign, target bounds, no-contact case, passive energy dissipation and fixed-step convergence. Further math: [Physics model](../../docs/physics.md).
