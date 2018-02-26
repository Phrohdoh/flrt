# `flrt`

`flrt`, pronounced "flirt", is a command-line tool that makes it easy to add or remove dependencies for a given `pubspec.yaml` file

## Project Status

Brand new. Not ready for use.

## Build `flrt`

```sh
$ cargo build
```

## Run `flrt` via `cargo`

```sh
$ cargo run -- <package name>
```

The above will work if there is a `pubspec.yaml` file in the current working directory.

If that is not the case you will need to provide the path to a valid manifest.

```sh
$ cargo run -- <package name> --manifest-path /path/to/a/pubspec.yaml
```