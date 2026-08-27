# Security checklist

- [ ] No secrets or credentials committed.
- [ ] No remote service required for normal file processing.
- [ ] Source paths are handled as untrusted input.
- [ ] Destination paths are validated natively.
- [ ] Generated names are sanitized for Windows.
- [ ] Symlink/reparse-point behavior is explicitly tested on Windows.
- [ ] Errors are surfaced without silently continuing unsafe operations.
- [ ] Logs do not contain unnecessary sensitive path data when privacy mode is enabled.
