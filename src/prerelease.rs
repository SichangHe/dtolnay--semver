macro_rules! compatible_prerelease_position {
    (
        $comparator_major:expr,
        $comparator_minor:expr,
        $comparator_patch:expr,
        $comparator_has_prerelease:expr,
        $version_major:expr,
        $version_minor:expr,
        $version_patch:expr $(,)?
    ) => {
        $comparator_major == $version_major
            && $comparator_minor == Some($version_minor)
            && $comparator_patch == Some($version_patch)
            && $comparator_has_prerelease
    };
}
