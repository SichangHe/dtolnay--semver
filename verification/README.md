# verified prerelease admission gate

scope

- property `prerelease-compatible-position`
  - a comparator can admit a prerelease version through the prerelease gate exactly when its major, minor, and patch equal the version's and the comparator prerelease is nonempty
- public operations
  - `Comparator::matches`
  - `VersionReq::matches`
- production path
  - `eval::matches_comparator` or `eval::matches_req`
  - `eval::pre_is_compatible`
  - `compatible_prerelease_position!`, expanded directly at the production call site and in the verifier
- excludes comparator ordering, operators, parsing internals, build metadata, and caret compatibility

gates

- these are manual pre-handoff gates; this bounded experiment does not add CI provisioning
- adversarial public-path search runs before the proof and covers matching numeric tuples, mismatched tuples, empty comparator prereleases, and requirement conjunctions
- Verus `0.2025.11.30.840fa61`, supplied as `VERUS`, must check the shared executable numeric kernel
- a Rust correspondence test must compare the public API against an independent oracle
- an independent reviewer must inspect the final diff and rerun the stated gates

assumption

- `ASSUM.md` defines `prerelease_gate_scope`
