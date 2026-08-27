# Project handoff

The repository is intentionally structured so the safety-critical file engine is independent from the visual preview.

The next implementation work must be code, not additional UI-only simulation:

- establish Tauri desktop shell;
- expose typed commands to Rust;
- connect source/destination pickers;
- execute operation plans through the Rust core;
- stream progress events;
- persist journal state;
- implement startup recovery;
- generate reports;
- add Windows packaging.

Do not duplicate filesystem copy logic in TypeScript/React.
