import os

OS_APPLICATION_TXT = "../os/application.txt"

apps = ["init", "terminal", "virtio-blk", "fs"]

with open(OS_APPLICATION_TXT, "w") as f:
    f.write(" ".join(apps))
