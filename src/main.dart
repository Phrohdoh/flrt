import 'dart:async';
import 'dart:io';
import 'package:http/http.dart' as http;
import 'package:bwu_pub_client/bwu_pub_client.dart' show PubPackage, PubClient;
import 'package:pubspec/pubspec.dart';
import 'package:args/command_runner.dart';

Future main(List<String> arguments) async {
  var _ = new CommandRunner('flrt', 'Allows addition/removal of dependencies to/from a `pubspec.yaml` file')
    ..addCommand(new AddCommand())
    ..addCommand(new RemoveCommand())
    ..run(arguments);
}

Future<PubPackage> queryForPackage(String name) async {
  var client = new PubClient(new http.Client());
  PubPackage pkg = null;

  try {
    pkg = await client.fetchPackage(name);
  } on FormatException catch(e) {
    // This is pretty ugly but, AFAICT, `fetchPackage` assume success because it goes directly into JSON parsing.
    var c = new Completer();
    c.complete(null);
    return c.future;
  }

  return pkg;
}

class AddCommand extends Command {
  @override
  String get description => 'add dependencies to a given pubspec.yaml file';

  @override
  String get name => 'add';

  AddCommand() {
    argParser.addOption("manifest-directory", defaultsTo: ".");
  }

  Future<Null> run() async {
    var manifest_dir = argResults['manifest-directory'];
    var d = new Directory(manifest_dir);
    PubSpec spec = null;

    try {
      spec = await PubSpec.load(d);
    } on FileSystemException catch (e) {
      stderr.writeln("Error attempting to load ${e.path}: ${e.message}");
      stderr.writeln("Are you sure `${e.path}` exists? (consider using the `--manifest-directory` option)");
      exitCode = 1;
      return;
    }

    var newDeps = spec.dependencies;

    for (var package_name in argResults.rest) {
      if (!spec.dependencies.containsKey(package_name)) {
        var pkg = await queryForPackage(package_name);
        if (pkg == null) {
          stderr.writeln("Could not find package `$package_name` in pub registry");
          exitCode = 2;
          return;
        }

        var ver = pkg.latest.version.toString();
        newDeps[package_name] = ver;
      }
    }

    spec = spec.copy(dependencies: newDeps);
    spec.save(d);
  }
}

class RemoveCommand extends Command {
  @override
  String get description => 'remove dependencies from a given pubspec.yaml file';

  @override
  String get name => 'remove';

  RemoveCommand() {
    argParser.addOption("manifest-directory", defaultsTo: ".");
  }

  Future<Null> run() async {
    var manifest_dir = argResults['manifest-directory'];
    var d = new Directory(manifest_dir);
    var spec = await PubSpec.load(d);

    var newDeps = spec.dependencies;

    for (var package_name in argResults.rest) {
      if (spec.dependencies.containsKey(package_name)) {
        newDeps.remove(package_name);
      }
    }

    spec = spec.copy(dependencies: newDeps);
    spec.save(d);
  }
}
