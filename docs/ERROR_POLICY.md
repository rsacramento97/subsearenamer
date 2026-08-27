# Error policy

Errors are classified as recoverable or blocking.

## Blocking

- invalid source;
- source/destination overlap;
- destination already exists;
- insufficient disk space;
- read/write failure;
- validation mismatch;
- invalid destination name;
- ambiguous timestamp/fuso;
- interrupted operation without a safe recovery state.

A blocking error must not be converted into success by the UI.

## Recoverable

A recoverable error may allow the operator to retry the item only after the underlying condition is resolved. Retry must create a fresh temporary destination and must never reuse an unverified partial as a completed file.
