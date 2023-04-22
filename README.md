## ipa_renamer

A command line tool for renaming your ipa files quickly and easily.

### Usage
```bash
ipa_renamer 0.0.1
A command line tool for renaming your ipa files quickly

USAGE:
    ipa_renamer.exe [OPTIONS] <GLOB> [TEMPLATE]

ARGS:
    <GLOB>        file path pattern to rename
    <TEMPLATE>    Name template for the new file [default: $raw@$CFBundleIdentifier]

OPTIONS:
    -h, --help           Print help information
    -o, --out <OUT>      The dir to save the renamed files [default: renamed]
    -t, --temp <TEMP>    The temp dir for the extracted ipa file [default: ./temp]
    -V, --version        Print version information
```
### Run from source

#### Prerequisites

- Rust: https://www.rust-lang.org/tools/install

#### Run

```bash
$ cargo run
```

#### Build

```bash
$ cargo build
```

### Example
```bash
$ ./ipa_renamer './ipa/*'
[skipped] ipa/16756024.plist is not a ipa file
[renamed] ipa/DumpApp_1.0.4.ipa to renamed/DumpApp_1.0.4@com.dumpapp.ipa
[renamed] ipa/Soulver_2.8.3APP喵砸壳.ipa to renamed/Soulver_2.8.3APP喵砸壳@com.acqualia.soulver-iPhone.ipa
[renamed] ipa/aszs.ipa to renamed/aszs@rn.notes.best.ipa
[renamed] ipa/thor134.ipa to renamed/thor134@com.pixelcyber.dake.thor.ipa
[Done!]
```
