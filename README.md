# `flrt`

`flrt`, pronounced "flirt", is a command-line tool that makes it easy to add or remove dependencies for a given `pubspec.yaml` file

## Status

Brand new. Might eat your homework.

## Goals

- To be sunsetted by the addition of `flutter packages add <package name>` and `flutter packages remove <package name>` commands
- To wrap `flutter` and make dependency manipulation easier

## Run `flrt` via `dart`

```sh
$ dart src/main.dart <command> <packages>
```

The above will work if there is a `pubspec.yaml` file in the current working directory.

If that is not the case you will need to provide the path to a directory containing a valid manifest.

```sh
$ dart src/main.dart <command> --manifest-directory /path/to/a/directory <packages>
```

## Example invocations

Assuming you have a `./pubspec.yaml` file with the following contents.

```yaml
dependencies: 
  args: "1.4.1"
  pubspec: "0.0.15"
```

Run the following command.

```sh
$ dart src/main.dart add path_provider
```

Observe that your `./pubspec.yaml` file has been modified like so.

```yaml
dependencies: 
  args: "1.4.1"
  pubspec: "0.0.15"
  path_provider: <unknown>
```

This demonstrates the `add` command. The inverse of that command is `remove`.

Again given the initial manifest.

```yaml
dependencies: 
  args: "1.4.1"
  pubspec: "0.0.15"
```

```sh
$ dart src/main.dart remove pubspec
```

Will modify the manifest like so.

```yaml
dependencies: 
  args: "1.4.1"
```

## License

See the [LICENSE](./LICENSE) file.