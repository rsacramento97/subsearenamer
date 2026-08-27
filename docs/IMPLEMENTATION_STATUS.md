# Implementation status

## Safety baseline

The native Rust copy engine has protections for source immutability, destination conflicts, tree overlap, disk space, temporary-file uniqueness, size validation, optional SHA-256 verification and cleanup of failed partials.

## Product boundary

The web layer is a preview/planning surface. The remaining production work is the native Windows integration and real-world validation.

## Release rule

No production release until the Windows integration passes the documented release gates and QA sign-off.
