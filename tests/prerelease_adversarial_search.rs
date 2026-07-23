use semver::{Comparator, Op, Version, VersionReq};

fn compatible_prerelease_position(comparator: &Comparator, version: &Version) -> bool {
    comparator.major == version.major
        && comparator.minor == Some(version.minor)
        && comparator.patch == Some(version.patch)
        && !comparator.pre.is_empty()
}

fn comparator_matches_impl_oracle(comparator: &Comparator, version: &Version) -> bool {
    let comparator_position = (
        comparator.major,
        comparator.minor.expect("full comparator"),
        comparator.patch.expect("full comparator"),
        &comparator.pre,
    );
    let version_position = (version.major, version.minor, version.patch, &version.pre);
    let ordering = version_position.cmp(&comparator_position);
    let operation_matches = match comparator.op {
        Op::GreaterEq => ordering.is_ge(),
        Op::LessEq => ordering.is_le(),
        _ => unreachable!("bridge uses only >= and <= comparators"),
    };

    operation_matches
}

fn comparator_oracle(comparator: &Comparator, version: &Version) -> bool {
    comparator_matches_impl_oracle(comparator, version)
        && (version.pre.is_empty() || compatible_prerelease_position(comparator, version))
}

fn requirement_oracle(requirement: &VersionReq, version: &Version) -> bool {
    requirement
        .comparators
        .iter()
        .all(|comparator| comparator_matches_impl_oracle(comparator, version))
        && (version.pre.is_empty()
            || requirement
                .comparators
                .iter()
                .any(|comparator| compatible_prerelease_position(comparator, version)))
}

#[test]
fn prerelease_gate_rejects_every_other_numeric_position() {
    let comparator = Comparator::parse(">=1.1.1-alpha").unwrap();

    for major in 0..=2 {
        for minor in 0..=2 {
            for patch in 0..=2 {
                for prerelease in ["alpha", "beta", "rc.1"] {
                    let version =
                        Version::parse(&format!("{major}.{minor}.{patch}-{prerelease}")).unwrap();
                    assert_eq!(
                        comparator.matches(&version),
                        comparator_oracle(&comparator, &version),
                        "{version}"
                    );
                }
            }
        }
    }
}

#[test]
fn prerelease_gate_requires_an_explicit_prerelease_comparator() {
    let comparator = Comparator::parse("<=1.1.1").unwrap();
    let version = Version::parse("1.1.1-alpha").unwrap();

    assert_eq!(
        comparator.matches(&version),
        comparator_oracle(&comparator, &version)
    );
}

#[test]
fn requirement_needs_one_compatible_prerelease_comparator() {
    let version = Version::parse("1.1.1-beta").unwrap();
    let incompatible = VersionReq::parse(">=1.1.0-alpha, <=2.0.0-alpha").unwrap();
    let compatible = VersionReq::parse(">=1.1.1-alpha, <=2.0.0-alpha").unwrap();

    assert_eq!(
        incompatible.matches(&version),
        requirement_oracle(&incompatible, &version)
    );
    assert_eq!(
        compatible.matches(&version),
        requirement_oracle(&compatible, &version)
    );
}
