# Test matrix

| Area | Required test | Expected result |
|---|---|---|
| Original | compare bytes before/after | identical |
| Duplicate | destination already exists | blocked, no overwrite |
| Overlap | destination inside source | blocked |
| Space | insufficient free space | blocked before copy |
| Copy failure | simulated write failure | partial removed |
| Hash | mismatched SHA-256 | final destination not created |
| Cancel | cancel during copy | original untouched, no false success |
| Disconnect | remove destination media | operation fails safely |
| Unicode | accented/Unicode filename | valid sanitized name |
| Windows reserved | CON/NUL/COM1 etc. | blocked/sanitized |
| Long path | near Windows path limit | clear validation error |
| Timezone | UTC offsets changed | only planned filename changes |
| Batch | duplicate generated names | conflict shown before execution |
| Restart | interrupted journal | completed items recognized safely |
| Offline | no network | file processing still works |
