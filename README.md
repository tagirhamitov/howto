# HowTo

## Installation
1. Install rust using [rustup](https://rustup.rs/).
1. Clone this repository.
1. Run `cargo install --path PATH_TO_CLONED_REPOSITORY`.
1. Put your openai token to the file `.howto` in your home directory.

## Usage
It's better to show by examples:

```bash
$ howto print current date
date
```

```bash
$ howto print username
echo $USER
```

```bash
$ howto copy all text from file in vim
To copy all text from a file in Vim, you can use the command ":1,$y" followed by the desired register name to store the text (e.g. "*y).
```

```bash
$ howto print last commit message
git log -1 --pretty=%B
```
