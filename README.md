[![IniEd](ICON.png)](https://www.medo64.com/inied/)  IniEd
==========================================================

Command line .ini file editor.


## Goals ##

This project has the following goals:

* Support for wide range of .ini file formats (e.g. both # and ; for comment).

* No unintended modifications. Lines that are not modified will be written
  without unnecessary corrections.


## Usage ##

    inied [OPTIONS] <file>

|      | Option           | Value       | Description                                            |
|------|------------------|-------------|--------------------------------------------------------|
| `-s` | `--section`      | `<section>` | Section to show or replace                             |
| `-k` | `--key`          | `<key>`     | Key to show or replace                                 |
| `-p` | `--print`        |             | Show value only                                        |
| `-r` | `--print-raw`    |             | Value will not have quotes removed upon printing       |
| `-d` | `--delete`       |             | Specified entry or section will be deleted             |
| `-a` | `--append`       | `<value>`   | Value will be appended                                 |
| `-c` | `--change`       | `<value>`   | Value will be changed only if it exists                |
| `-e` | `--edit`         | `<value>`   | Value will be changed if exists or added if it doesn't |
|      | `--pretty-print` |             | Format output to look nicer                            |
|      | `--trim`         |             | Trim leading and trailing spaces                       |
|      | `--no-comments`  |             | Remove all comments                                    |
| `-i` | `--in-place`     |             | Writes content back to the same file after processing  |
| `-v` | `--verbose`      |             | Sets the level of verbosity                            |
| `-h` | `--help`         |             | Prints help information                                |
| `-V` | `--version`      |             | Prints version information                             |
|      |                  | `[file]`    | File to process                                        |

| Exit code | Description        |
|-----------|--------------------|
| 0         | Success            |
| 1         | Input read error   |
| 2         | Output write error |
| 255       | Argument error     |


## Examples ##

### Showing value ###

To show just the value, we need to specify `--section`, `--key`, and `--print`.

    inied --section mysqld --key key_buffer --print  examples/my.cnf

### Editing value ###

To edit key to a particular value, you need to specify `--section`, `--key`, and
`--edit`. If value is not present it will be appended to the end of section.

    inied --section mysqld --key key_buffer --edit 200M  examples/my.cnf

### Changing value ###

To change key to a particular value, you need to specify `--section`, `--key`,
and `--change`. If value is not present, nothing will happen.

    inied --section mysqld --key key_buffer --edit 200M  examples/my.cnf

### Appending value ###

To append key with a particular value, you need to specify `--section`, `--key`,
and `--append`. Value will be appended to the end of section.

    inied --section mysqld --key key_buffer --edit 200M  examples/my.cnf

### Deleting key ###

To delete the key, one has to specify `--section`, `key`, and `--delete`.

    inied --section mysqld --key key_buffer --delete  examples/my.cnf

### Deleting section ###

To delete the whole section, one has to specify only `--section` and `--delete`.

    inied --section mysqld --delete  examples/my.cnf

### Showing section ###

To show a single section, one has to specify only `--section`.

    inied --section mysqld  examples/my.cnf

### Pretty print ###

To clean up output, one can use `--pretty-print`.

    inied --pretty-print  examples/my.cnf

### Removing comments ###

To remove all comments, one can use `--no-comments`.

    inied --no-comments  examples/my.cnf

### In-place changes ###

To do all those changes directly to a file, one can use `--in-place`.

    inied --pretty-print --in-place  examples/my.cnf

### Chaining calls ###

As `inied` supports reading from standard input and writing to standard output,
multiple calls can be chained together.

    inied --section mysqld examples/my.cnf | inied --key key_buffer | inied --print
