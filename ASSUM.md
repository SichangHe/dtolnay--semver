# prerelease_gate_scope

The verified kernel represents a comparator's prerelease as one boolean: nonempty or empty.
It proves only the numeric-position admission gate used before a prerelease version matches.
The production parser remains responsible for valid prerelease text and comparator shape.
The public correspondence tests construct values through `Comparator::parse` and `Version::parse`.
