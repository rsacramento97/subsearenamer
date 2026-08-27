# QA sign-off

A human operator must sign off the Windows release after automated checks pass.

Minimum evidence:

- source file hash before operation;
- destination file hash after operation;
- source file remains unchanged;
- destination naming matches the approved preview;
- no partial files remain;
- report identifies success/failure per item;
- interruption test completed;
- insufficient-space test completed;
- duplicate-name test completed;
- clean-machine Portable test completed;
- clean-machine installer test completed.

Automated CI alone is not sufficient evidence for production release because CI does not exercise the operator's actual Windows storage environment.
