# Rotire

![Rotire Logo](./rotire.png)

Rotire is a small tool that allows you to rotate files.

You can either archive the files or delete them.

# Options

Rotire has the following options:

```
Simple program to rotate files

Usage: rotire [OPTIONS] --directory <DIRECTORY> [ACTION]

Arguments:
  [ACTION]  Select the action rotire should run. [default: archive] [possible values: archive, delete]

Options:
  -d, --directory <DIRECTORY>          Path of the directory on which rotire should run
  -k, --keep-n <KEEP_N>                How many items to keep, defaults to 4 [default: 4]
  -p, --prefix-filter <PREFIX_FILTER>  Only apply action on the file names matching the prefix
  -s, --suffix-filter <SUFFIX_FILTER>  Only apply action on the file names matching the suffix
  -h, --help                           Print help
  -V, --version                        Print version
```

The following command archives all the files in the `not_in_train` directory and keeps the most recent four
files unarchived.

```angular2html
rotire --directory /home/denis/Pictures/not_in_train
```