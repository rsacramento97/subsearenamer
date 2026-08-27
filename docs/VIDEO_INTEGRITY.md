# Video integrity policy

SubSeaRenamer is a file-copy and naming tool, not a video-processing tool.

The application must treat every input video as immutable source data. It must not decode, re-encode, remux, trim, normalize, repair, rewrite metadata, or otherwise transform the video as part of renaming.

A successful operation means only that a byte-preserving copy was created at the requested destination and passed the configured validation.
