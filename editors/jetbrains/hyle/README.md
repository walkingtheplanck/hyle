# Hyle IntelliJ Plugin

This plugin registers `.hyle` files and provides syntax highlighting for the Hyle simulation language.

## Prerequisites

- Java 17 or newer.
- Gradle 9.0 or newer.

If Java is not installed system-wide on macOS, you can use the JetBrains Runtime bundled with an installed JetBrains IDE:

```sh
export JAVA_HOME="$HOME/Applications/RustRover.app/Contents/jbr/Contents/Home"
```

## Build

From this directory:

```sh
gradle build
```

The packaged plugin ZIP is written under `build/distributions/`.

## Test

Run the lexer unit test:

```sh
gradle test
```

Run IntelliJ with the plugin loaded in a sandbox:

```sh
gradle runIde
```

Then create or open a `.hyle` file and verify that file association, icon, comments, directives, keywords, numbers, strings, and operators are highlighted.

## Verify

Run the IntelliJ Platform plugin checks:

```sh
gradle verifyPluginProjectConfiguration
gradle verifyPlugin
```
