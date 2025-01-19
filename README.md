# SRM (Safely Remove)

[!CAUTION] I'm still working on this project, so it's not ready for use yet.

**srm** is a command-line program to delete files securely.

## Motivation

The `rm` command is the most commonly used command to delete files in Unix-like operating systems. However, it's really easy to accidentally delete important files with it, and never be able to restore it.

`srm` is a safer alternative to `rm` that stores deleted files in a trash directory instead of deleting them permanently.

## Roadmap

- [x] Basic functionality, like storing deleted files in a trash directory, restoring them, and deleting them permanently.
- [ ] Implementing the `-r` option to delete directories recursively.
- [ ] Daemon mode to monitor the trash directory and delete files permanently after a certain period of time.
