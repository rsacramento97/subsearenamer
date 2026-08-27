# Desktop Architecture

## Target
Windows 10/11 desktop application, with installer and portable distribution.

## Separation
- Web/Lovable: preview and UI development.
- Desktop shell: Tauri.
- Native core: Rust for filesystem operations and integrity checks.

## Safe file pipeline
1. Validate source and destination are different.
2. Verify destination free space.
3. Create a dedicated destination folder such as `RENAMED_VIDEOS`.
4. Copy each original to a temporary file in the destination.
5. Flush/sync the temporary file.
6. Validate byte length.
7. Optionally calculate and compare SHA-256.
8. Atomically rename the validated temporary copy to the final filename.
9. Persist operation state and log entry.
10. Never modify or delete the source.

## Recovery
Operations are resumable. A temporary/incomplete destination file is never reported as completed. On restart, the application reconciles the journal with the filesystem and continues only pending items.

## Non-goals
The native pipeline must not decode, re-encode, convert, remux, or otherwise rewrite video contents.
