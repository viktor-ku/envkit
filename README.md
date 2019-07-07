# envkit

> The stupid .env files management tool

Try the following command to get you started:

```
envkit --help
```

## Commands

### Diff

Shows which variables are not found in the file b compared to the file a. It checks for the given
variable across the entire file so it doesn't have to be on the same line.

Consider next two files:

```
# File A.env
ONE=1
TWO=2
THREE=3
```

and

```
# File B.env
THREE=3
FOUR=4
FIVE=5
```

`envkit diff A.env B.env` will output:

```
Next variables were found in file_a, but not in file_b:
  ONE (line 2)
  TWO (line 3)
```

while `envkit diff B.env A.env` will output:

```
Next variables were found in file_a, but not in file_b:
  FOUR (line 3)
  FIVE (line 4)
```

Note that it does not matter that `THREE` is located at the third line in the file `A.env` but on the
first line in the file `B.env`.

PS. Output will be pretty and informative very soon :P

## Install

At the moment you have to build it from source. You need `rustup` for it.

Building process should be as easy as `cargo build --release`.
