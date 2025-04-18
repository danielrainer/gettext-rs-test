# gettext-rs test

This repo provides a small example on how gettext-rs can be used to achieve translation in a Rust program.

## Setup

Running this requires having gettext installed. (The runtime libs, msgmerge, msgfmt)

The setup is done in `build.rs`, which just runs a couple of commands which could also be run manually in a shell.

`cargo run` should suffice to run the program and see output according to the `LANG` environment variable, provided translations for this language exist. At the moment, only German translations are included.

Below is a description how the setup could be run manually, without needing `build.rs`:

```sh
# first time only, could be replaced by xgettext
cargo install xtr

# generate messages.po
cargo xtr src/main.rs

# update german translations in messages-de.po
# (line numbers are updated, and new strings added if there are any)
msgmerge --update --no-fuzzy-matching --no-wrap messages-de.po messages.po

# create the messages.mo file
msgfmt --check-format messages-de.po

# put the file into a place where it can be found at runtime
cp messages.mo locale/de/LC_MESSAGES/hellorust.mo

# set your LANG environment variable, e.g.
LANG=de_DE.utf8
export LANG

# run the program
# The output should now use the translations for the corresponding language (German in this example), if translations exist.
cargo run
```
