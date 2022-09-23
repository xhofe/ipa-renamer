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