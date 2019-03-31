ensync
======

This projects goal is for primarily learning rust. The end goal is to be a program which takes a source directory, recursively iteratoes through it's members and send it's encrypted contents with filenames hashed to a target. Eventually we should be able to send incrementals (much like rsync) to the target. Eventually the end goal of the project will be to make encrypted backups on untrusted devices and only send deltas on preestablished targets.

### Usages

```bash
ensync path/to/backup target secret
```