# SubSeaRenamer — Test Plan

## Safety gates
- Original files must have identical byte hashes before and after every operation.
- Destination collisions must never overwrite.
- Insufficient disk space must stop before copying.
- Failed or interrupted copies must never be marked complete.
- Partial destination files must be detectable and resumable/cleanable.

## Functional tests
1. One MP4.
2. 10 MP4 files.
3. 1,000+ files.
4. Large files and external drives.
5. Duplicate generated names.
6. Existing destination names.
7. Unicode/special characters.
8. Long paths/names.
9. Multiple extensions.
10. UTC offsets including UTC-03:00.
11. Manual offset.
12. Sequence numbering.
13. Simulation mode.
14. Cancel during copy.
15. Disconnect source/destination drive.
16. Resume after process termination.
17. SHA-256 mismatch handling.
18. Insufficient free space.
19. Undo destination operation.
20. Export report.

## Release gate
A desktop release is not considered production-ready until automated and manual tests demonstrate that originals remain unchanged and that all successful destination copies are byte-identical to their sources.
