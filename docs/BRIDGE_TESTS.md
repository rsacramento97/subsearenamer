# Native bridge tests

The desktop bridge must be tested for:

- valid plan reaches Rust core;
- invalid source is rejected;
- source/destination overlap is rejected;
- existing destination is rejected;
- cancellation never reports success;
- copy failure never reports completion;
- hash mismatch never reports completion;
- restart can identify unfinished operations;
- UI errors cannot bypass the native safety core.
