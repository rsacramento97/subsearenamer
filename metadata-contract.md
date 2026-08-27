# Media metadata contract

The renamer must never infer a capture timestamp from the PC's local clock.

A concrete media adapter must provide:

- `captured_utc`: RFC3339 timestamp with an explicit offset;
- `source`: provenance of the timestamp (for example container metadata, stream metadata, or an external media probe).

If no trustworthy timestamp is available, the item must be marked `invalid` in preview and execution must skip it.

## Required safety rules

1. Never mutate the source during metadata inspection.
2. Never silently substitute filesystem creation/modification time for capture time.
3. Never silently interpret a timezone-less timestamp as local time.
4. Preserve the original extension.
5. Duplicate final names are a preview conflict, not an execution-time surprise.
