# Release gates

The following gates are mandatory and sequential:

1. **Core gate:** Rust tests pass.
2. **Planning gate:** naming/timezone/conflict tests pass.
3. **Bridge gate:** native UI bridge tests pass.
4. **Windows gate:** Portable and installer build successfully.
5. **Integrity gate:** real-file byte/hash validation passes.
6. **Recovery gate:** interruption/disconnect/retry behavior passes.
7. **QA gate:** human operator signs off.
8. **Release gate:** only then mark the GitHub release as production.

Failure at any gate blocks release.
