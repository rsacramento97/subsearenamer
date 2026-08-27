# Next implementation target

The next code milestone is the native desktop integration boundary:

1. Tauri shell.
2. Native file/folder pickers.
3. Bridge from UI operation plan to Rust core.
4. Per-file progress and cancellation.
5. Durable operation journal.
6. Recovery scan on startup.
7. Exportable operation report.
8. Windows Release build.

The bridge must pass structured commands and structured results; the UI must never implement a second, independent copy engine.
