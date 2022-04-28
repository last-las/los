import os

OS_APPLICATION_TXT = "../os/application.txt"

# apps = ["shell", "init", "terminal", "virtio-blk", "fs"]
apps = ["init", "terminal", "idle", "shell", "virtio-blk", "fs", "test_fs"]

with open(OS_APPLICATION_TXT, "w") as f:
    f.write(" ".join(apps))
