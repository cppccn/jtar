<!-- cargo-sync-readme start -->

# `JTAR`

_JTAR is a dumb archive format that appears to be a subset of JSON (which is handy)_

<!-- cargo-sync-readme end -->

Folders are `[ ... ]` array of files, where files are ` { ... } ` dictionaries with properties `metadata` and `content` which is base64 encoded.

## Is JTAR inefficient?

Yes and No: Archives are heavy (by ~ %) but after `gunzip`, the difference … more serious benchmark would be neat.

## Extensions

- laziness `-l` option
- chunk `-c` option
