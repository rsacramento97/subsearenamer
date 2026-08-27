# SubSeaRenamer — Desktop Architecture

## Separation of responsibilities
The Lovable/Vite React application is the preview and UI layer. Real file operations for production must run in a native desktop layer.

## Recommended stack
- UI: React + TypeScript + Vite
- Desktop shell: Tauri (preferred for a small portable Windows binary) or Electron if ecosystem requirements demand it
- Native file engine: Rust for Tauri, with atomic/validated copy operations
- Local project/log storage: JSON/SQLite, never video content

## Safe production pipeline
1. Inspect source file without modifying it.
2. Resolve destination outside the source directory.
3. Check free space.
4. Create a temporary destination filename.
5. Stream-copy bytes without decoding/re-encoding.
6. Verify byte count.
7. Optionally compute SHA-256 source and temporary copy.
8. If and only if validation passes, atomically rename the temporary copy to its final name.
9. Append an immutable operation record to the local log.
10. Never delete or alter the source.

## Crash safety
Temporary files must use a recognizable suffix such as `.subsearenamer.partial`. An interrupted run must leave originals untouched and allow safe resume/cleanup of partial destination copies.

## Collision policy
Destination collisions must never overwrite. The engine should return an explicit conflict and let the UI choose skip, alternate sequence, or manual resolution.

## Offline requirement
The desktop application must not upload videos or telemetry containing file contents. All processing is local.

## Portable distribution
The final release should support a Windows Portable package and a conventional installer. The installer must not require cloud services for core functionality.
