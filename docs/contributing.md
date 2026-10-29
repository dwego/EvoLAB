# Contributing to EvoLab

Thank you for helping build a transparent artificial-life research playground.

## Development setup

~~~bash
git clone https://github.com/dwego/EvoLAB.git
cd EvoLAB
cargo fmt --check
cargo check
cargo test
cargo run
~~~

Avoid installing a crates.io dependency merely because a local module has the same name (e.g. \`physics\`); local module paths are declared with \`mod physics;\`.

## Working on a feature

1. Start from an up-to-date branch.
2. Write a concise issue or design note specifying expected behavior, invariants and failure modes.
3. Keep changes limited to one responsibility (brain, physics, world, graphics, creatures).
4. Add a test that does not require visual inspection when practical.
5. Run format/check/test; for GPU changes, run locally and capture hardware/software context.
6. Update documentation and screenshots/diagrams if behavior changes.
7. State clearly which effects are verified and which are hypothetical.

## Commit style

Suggested Conventional Commits:

- \`feat(physics): add constrained hinge actuator\`
- \`fix(graphics): show neural grip signals\`
- \`docs: describe simulation architecture\`
- \`test(brain): verify bounded outputs\`
- \`refactor(world): isolate episode initialization\`

## Documentation conventions

- Keep physical quantities and units explicit.
- Prefer mathematical equations in GitHub Markdown \`$$...$$\`.
- Mark planned features clearly as proposed.
- Link SVG diagrams relatively so documentation works in forks.
- Avoid presenting a conceptual illustration as a runtime screenshot.
- Check links and Mermaid/SVG previews in the GitHub renderer.

## Testing philosophy

- **Pure unit tests:** network forward pass, genotype mutation, vector force math.
- **Deterministic integration tests:** passive spring, conservation sanity, contact conditions.
- **Experimental reproducibility:** same seed and configuration yield expected metrics within defined tolerances.
- **Visual smoke tests:** camera, mesh and debug presentation.

## Physics integrity

Do not directly change an organism's position to make it appear to walk. Motor commands should produce forces or controlled physical constraints. If an approximation is introduced, document its equations and limitations.

## Licensing

No project license is committed at this stage. Contributors should confirm the chosen license before submitting substantial contributions; the repository's public availability alone does not grant a specific reuse license.
