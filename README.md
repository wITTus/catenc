# Catenc
Category/Label encoder for the shell written in Rust.

## What is it?

In Machine Learning categories or labels in datasets often need to be encoded to numerical values.

Usually this is done by using something like Scikit-Learn's `sklearn.preprocessing.LabelEncoder` for Python.

This command line program is an attempt to do it on the shell.

## Setup

```
git clone git@github.com:wITTus/catenc.git
cd catenc

cargo build --release

target/release/catenc --help
```

## Example

Suppose you have a dataset like the following:

```
DOG;MALE;4
CAT;FEMALE;6
BIRD;FEMALE;2
DOG;FEMALE;7
DOG;FEMALE;1
CAT;FEMALE;6
DOG;MALE;3
```

### Encode entire lines

```
$ cat PETS | ./catenc
0
1
2
3
4
1
5
```

### Encode the first two of three categories:

```
$ cat PETS | ./catenc -k 1,2 -t';'
0;0;4
1;1;6
2;1;2
0;1;7
0;1;1
1;1;6
0;0;3
```

### Features

* Encode to integers as either text or Base64 (`-e`) encoded values
* Encode entire lines or individual columns
* Configurable column separator
* Written in Rust 😉
