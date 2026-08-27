# Native bridge contract

The desktop UI must communicate with the native core through typed operations.

## Request

- source path
- destination path
- destination name
- verification mode
- operation identifier
- cancellation token/reference

## Progress

- operation identifier
- source path
- destination path
- state
- bytes copied
- total bytes
- optional SHA-256
- error code/message when failed

## Rules

The UI cannot directly copy, move, rename or delete source videos. All destructive-capable filesystem operations must remain behind the native safety core.
