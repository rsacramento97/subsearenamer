# File handling rules

## Source

Read-only. Never rename, move, delete, truncate, rewrite, or replace.

## Temporary destination

Created with exclusive creation semantics. A collision creates a new temporary name rather than replacing anything.

## Final destination

Must not exist at finalization time. It is created only after copy, sync and validation succeed.

## Failure

Remove only the temporary artifact created by the current operation. Never remove an existing user file as part of error recovery.
