# Continuity after interruption

Every operation must have an identifier and a durable state. On application restart, the app should inspect its journal and classify items as:

- `COMPLETED`: final destination exists and passed validation;
- `IN_PROGRESS`: interrupted or uncertain; never assume success;
- `FAILED`: known failure with cleanup result;
- `UNKNOWN`: insufficient evidence; require operator review.

Only a validated destination can be classified as completed. An existing file with no validation evidence must not be trusted merely because its name matches the plan.
