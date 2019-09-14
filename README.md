# The Archivist

An idiot-proof archive management utility

[![GitHub Actions](https://github.com/raftario/archivist/workflows/Tests/badge.svg)](https://launch-editor.github.com/actions?workflowID=Tests)

## About

The Archivist is a command line utility targeted at archive management and related tasks. Its main goal is to be as intuitive to use as possible. People shouldn't have to google how to extract an archive, and ideally shouldn't even have to use the man page. At least I don't think I should, and that's why I made this (no, I can't remember `tar` commands). See for yourself.

```shell
$ acv compress file.txt file.txt.gz
```

Here, `acv` understands that you want to compress `file.txt` using the gzip algorithm with a default compression level, and that if `file.txt.gz` already exists it should be renamed.

```shell
$ acv decompress file.txt.gz
```

Here, it understands that you want to decompress `file.txt.gz`, still using the gzip algorithm, to `file.txt`, and still not overwrite it.

If you need more help with usage, just use the `-h` or `--help` flag, or the `help` subcommand.

## Features

### Archiving

- [ ] zip
- [ ] tar

### Compression

- [x] gzip (gz)
- [x] lzma (xz)
- [x] bzip2 (bz2)

### Hashing

- [ ] SHA3
- [ ] SHA2
- [ ] SHA1
- [ ] MD5
- [ ] BLAKE2
- [ ] RIPEMD-160

### PGP

- [ ] Verify
- [ ] Verify (detached)
- [ ] Sign
- [ ] Sign (detached)

### Nice to have

- [ ] Customisable aliases
- [ ] VirusTotal scanning
- [ ] TUI
- [ ] man pages
- [ ] Shell completion scripts
- [ ] Website

## Licensing

The Archivist itself (the `acv` binary) is licensed under the GPLv3+ (see [LICENSE](LICENSE)). Each subcrate is licensed under either the MIT license or the GPLv3, depending on the crate.