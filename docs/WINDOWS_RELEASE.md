# Windows release plan

## Portable

Build the desktop application in release mode and package all required runtime assets beside the executable. The portable package must not require network access for normal processing.

## Installer

Installer must:

- install without modifying user video files;
- support a clean-machine install test;
- create application directories only where required;
- keep logs outside video source folders;
- uninstall without deleting user videos or generated reports unless explicitly selected by the user.

## Gate

No installer or Portable artifact should be advertised as production-ready until the complete `docs/RELEASE_CHECKLIST.md` and `docs/TEST_MATRIX.md` have been executed on Windows.
