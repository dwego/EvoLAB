# Physics and motor model

![Spring mechanics](../assets/physics.svg)

> **Scope:** exact mechanics of the existing two-sphere crawler, plus explicit future alternatives. This is an engineering prototype, not validated biomechanics.

## 1. Coordinate system and body

Bevy/Avian use a 3D world with **Y upward**, gravity downward, and locomotion currently mainly along **X**. The ground collider in the published code is a box **200 × 0.2 × 40**, centered at \((0,-0.1,0)\), so its upper face is \(Y=0\).

There are two sphere bodies:

| Property | Left body | Right body |
|---|---:|---:|
| Radius | 0.25 m | 0.25 m |
| Mass | 1.0 kg | 1.0 kg |
| Starting position | \((-0.6,0.27,0)\) | \((0.6,0.27,0)\) |
| Body type | Dynamic | Dynamic |
| Rotation | Locked | Locked |
| Collider | Sphere | Sphere |
| Contact friction component | 0.0 | 0.0 |

The logical \`Creature\` holds their entity IDs. The Bevy/Avian physics components are the source of truth for position/velocity.

## 2. Spring geometry

Given positions \(\mathbf{x}_a\) and \(\mathbf{x}_b\):

$$
\mathbf{d}=\mathbf{x}_b-\mathbf{x}_a,\quad
\ell=\|\mathbf{d}\|,\quad
\hat{\mathbf d}=\frac{\mathbf d}{\ell}.
$$

The implementation skips a force step if \(\ell<10^{-4}\), to avoid division by zero.

Neural motor output \(u_m\in[-1,1]\) changes desired spring length:

$$
\ell_t=\max\left(0.3,\;\ell_0+A\,u_m\right).
$$

Current defaults:

$$
\ell_0=1.2\text{ m},\quad A=0.35\text{ m}.
$$

Thus (ignoring the lower clamp, which is inactive at current defaults):

$$
0.85\text{ m}\leq\ell_t\leq1.55\text{ m}.
$$

These are **target lengths**, not bounds on actual separation. The observed physical length can overshoot due to inertia, finite stiffness, time integration and contact.

## 3. Hooke's law and damping

Define axial extension \(e=\ell-\ell_t\). The restoring force acting on body **a** is

$$
\mathbf F_a =
\left[k e+c\,v_{\rm axial}\right]\hat{\mathbf d}
$$

where

$$
v_{\rm axial}=(\mathbf v_b-\mathbf v_a)\cdot\hat{\mathbf d},
$$

and body **b** receives its opposite by Newton's third law:

$$
\mathbf F_b=-\mathbf F_a.
$$

Current coefficients are \(k=70\ {\rm N/m}\) and \(c=5\ {\rm N\,s/m}\) in the source-defined model.

The damping sign is significant. If the bodies separate (\(v_{\rm axial}>0\)), damping pulls them together; if they approach, it resists approach. With positive \(c\), the damper dissipates relative kinetic energy.

A convenient interpretation of spring potential for a **fixed** target length is:

$$
U_s=\frac12 k(\ell-\ell_t)^2.
$$

But when \(\ell_t(t)\) is actively commanded by a motor, \(U_s\) is not a complete energy accounting: changing rest length injects/removes mechanical work.

## 4. Free MHS limit

If the target length is fixed, gravity/contact are absent, damping is zero, and only small axial displacements matter, the relative coordinate obeys:

$$
\mu\,\ddot e+k e=0,\qquad
\mu=\frac{m_a m_b}{m_a+m_b}.
$$

Hence

$$
\omega_n=\sqrt{\frac{k}{\mu}}
=\sqrt{k\left(\frac1{m_a}+\frac1{m_b}\right)}.
$$

For equal masses \(m_a=m_b=m=1\) kg:

$$
\omega_n=\sqrt{\frac{2k}{m}}
=\sqrt{140}\ \mathrm{rad/s}\approx11.83\ \mathrm{rad/s},
\quad
T_n\approx0.531\ \mathrm{s}.
$$

These are **idealized predictions**, not a claim that the current driven/contact-constrained simulation must display this period.

For two equal masses and a linear damper, the ideal axial relative equation is:

$$
\ddot e+\frac{2c}{m}\dot e+\frac{2k}{m}e=0.
$$

The damping ratio is

$$
\zeta=\frac{c}{\sqrt{2km}}.
$$

With the current parameters this is about \(0.423\), *for the ideal free oscillator only*. The real simulation includes commanded rest length and ground forces.

## 5. Gravity and normal contact

Avian handles gravity, rigid-body integration and contact. At rest on horizontal ground, a 1 kg sphere has \(mg\approx9.81\) N of gravitational load and a corresponding reaction if the contact is stable.

The height \(y\approx0.25\) observed in earlier telemetry shows contact geometry is behaving plausibly. It does **not** verify all contact dynamics.

## 6. The current locomotion mechanism (important!)

The working crawler did not advance reliably when only high/low \`Friction\` coefficients were toggled on spheres. The experiment temporarily set sphere friction to **zero** and applied a custom resistive horizontal force when each sphere is near the ground.

For grip command \(g_i\in[0,1]\):

$$
\mathbf v_{i,xz}=(v_{i,x},0,v_{i,z}),\qquad
\mathbf F_{{grip},i}
=\mathrm{clampMagnitude}\left(
- C g_i\,\mathbf v_{i,xz},F_{\max}g_i\right),
$$

where \(C=45\), \(F_{\max}=35\) in the current source.

The force is applied only if the body's center is at or below \(0.25+0.04=0.29\) m.

**Interpretation:** velocity-proportional resistive damping, capped in magnitude, acting as a simplified active ground interaction.

**Not**:
- a true no-slip constraint;
- real adhesion chemistry or a foot mechanism;
- static/dynamic Coulomb friction;
- a general-purpose model of deformable limbs.

For \(\mathbf v_{xz}\ne 0\), it opposes velocity and therefore dissipates energy. The grip force is **external** to the two-mass system, so it can change center-of-mass momentum. The internal spring force alone cannot do so.

## 7. Observed diagnostics and causal experiment

Previous exploratory runs found a nearly constant center X and spring length when the symmetric spheres were assigned strong contact grip. With physical friction disabled, length oscillated substantially while the center remained at \(x=0\), consistent with paired internal forces. With the custom asymmetric grip force restored, a visible forward displacement was observed.

This establishes a useful **prototype behavior**, not a validated locomotion mechanism. Future experiments must record signed displacement, contact status and energy expenditure.

## 8. Numerical stability and instrumentation

Essential planned tests:

1. **No-actuation control:** \(A=0\), compare settling time against damped oscillator expectations.
2. **Momentum:** remove ground/gravity and verify equal-and-opposite spring forces preserve center-of-mass momentum within numerical tolerance.
3. **Energy:** passive spring + damping should not increase total mechanical energy except for integrator error; separate work done by the actuator.
4. **Convergence:** compare results when the fixed time step is halved.
5. **Extreme genes:** reject non-finite input/output values, implausible force spikes and invalid collisions.
6. **Ground contact:** compare varying friction coefficients with geometric feet, not only spheres.

A fitness routine that rewards unrealistic numerical instability is measuring the **simulator's failure modes**, not necessarily intelligent behavior.

## 9. Future motors and contact models

Possible later models, all **not implemented**:

| Motor | Mechanism | Research question |
|---|---|---|
| Spring muscle | target rest length | can a soft actuator coordinate gait? |
| Joint torque | actuation about rotational axes | can a legged morphology balance? |
| Linear constraint motor | position/velocity target in a joint | how does stiffness affect control? |
| Foot contact | friction and support polygon | can stance/swing emerge physically? |
| Deformable body | many masses/springs or FEM | can morphology evolve elastic locomotion? |

For a scientific simulator, energy/work, actuator limits, solver tolerance, and contact model must be explicitly specified and tested.
