# SRM (Safely Remove)

**srm** is a command-line program to delete files securely.

## Motivation

The `rm` command is the most commonly used command to delete files in Unix-like operating systems. However, the `rm` command does not delete the file securely. The file can be recovered using data recovery tools. The `srm` command is used to delete files securely. The `srm` command overwrites the file with random data before deleting it. This makes it impossible to recover the file using data recovery tools.

## Roadmap

- [x] Basic functionality, like storing deleted files in a trash directory, restoring them, and deleting them permanently.
- [ ] Implementing the `-r` option to delete directories recursively.
- [ ] Daemon mode to monitor the trash directory and delete files permanently after a certain period of time.
