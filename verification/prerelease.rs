use vstd::prelude::*;

include!("../src/prerelease.rs");

verus! {

// @ASSUME:prerelease_gate_scope
spec fn compatible_prerelease_position_spec(
    comparator_major: u64,
    comparator_minor: Option<u64>,
    comparator_patch: Option<u64>,
    comparator_has_prerelease: bool,
    version_major: u64,
    version_minor: u64,
    version_patch: u64,
) -> bool {
    comparator_major == version_major
        && comparator_minor == Some(version_minor)
        && comparator_patch == Some(version_patch)
        && comparator_has_prerelease
}

fn verified_compatible_prerelease_position(
    comparator_major: u64,
    comparator_minor: Option<u64>,
    comparator_patch: Option<u64>,
    comparator_has_prerelease: bool,
    version_major: u64,
    version_minor: u64,
    version_patch: u64,
) -> (matches: bool)
    ensures matches == compatible_prerelease_position_spec(
        comparator_major,
        comparator_minor,
        comparator_patch,
        comparator_has_prerelease,
        version_major,
        version_minor,
        version_patch,
    ),
{
    compatible_prerelease_position!(
        comparator_major,
        comparator_minor,
        comparator_patch,
        comparator_has_prerelease,
        version_major,
        version_minor,
        version_patch,
    )
}

proof fn fully_specified_nonempty_characterization(
    comparator_major: u64,
    comparator_minor: u64,
    comparator_patch: u64,
    version_major: u64,
    version_minor: u64,
    version_patch: u64,
)
    ensures compatible_prerelease_position_spec(
        comparator_major,
        Some(comparator_minor),
        Some(comparator_patch),
        true,
        version_major,
        version_minor,
        version_patch,
    ) == (comparator_major == version_major
        && comparator_minor == version_minor
        && comparator_patch == version_patch),
{
}

}
