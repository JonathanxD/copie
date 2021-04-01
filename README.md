[![Doc](https://docs.rs/copie/badge.svg)](https://docs.rs/copie)
[![Crate](https://img.shields.io/crates/v/copie.svg)](https://crates.io/crates/copie)
[![Github Release](https://img.shields.io/github/v/release/JonathanxD/copie?label=github%20release)](https://github.com/JonathanxD/copie/releases)
[![License: MIT](https://img.shields.io/crates/l/validbr)](https://opensource.org/licenses/MIT)
# copie

**copie** a simple tool used by [Dracon IntelliJ Plugin](https://nest.pijul.com/Jonathan/Dracon) to copy content of files that should be opened in an editor through *$EDITOR* or *$VISUAL* application to a separated file, allowing them to be inspected and modified without sacrificing integration.

## How does copie works?

**copie** works by copying the file specified as first argument into the file specified in `COPIE_TO` environment variable, however, when `COPIE_FROM` is specified, the data of the file specified by this environment variable is copied into the file specified as first argument. And when both `COPIE_FROM` and `COPIE_TO` environment variables are set, the contents of the file of the first one is copied into the file of the second one.

Since 0.2.0, contents to write in the file could be specified in `COPIE_FROM_STRING` environment variable.

## Usage

Copies from `hello` to a new file `test`.

```fish
$ touch hello
$ echo 'Hello world' > hello
$ COPIE_TO=test copie hello 
```

Copies from `test` to the existing `hello`.

```fish
$ touch hello
$ echo 'Hello world' > hello
$ touch test
$ echo 'Hello world 2' > test
$ COPIE_FROM=test copie hello 
```

Copies from `test` into `hello`.

```fish
$ touch hello
$ echo 'Hello world' > hello
$ touch test
$ echo 'Hello world 2' > test
$ COPIE_FROM=test COPIE_TO=hello copie 
```

Write `copie` string into `hello`.

```fish
$ COPIE_FROM_STRING=copie COPIE_TO=hello copie 
```

Write `copie` string into `hello`.

```fish
$ COPIE_FROM_STRING=copie copie hello
```

## Practical use

**copie** is used in [Dracon IntelliJ Plugin](https://nest.pijul.com/Jonathan/Dracon) to extract record text and to send change text to `pijul`, the most common commands are:

#### Extract Record Change
```fish
COPIE_TO=/a/tmp/dir/RANDOM_HASH VISUAL="copie" pijul record
```

#### Send Record Change to Pijul Record
```fish
COPIE_FROM=/a/tmp/dir/RANDOM_HASH VISUAL="copie" pijul record
```
