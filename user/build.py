import os

OS_APPLICATION_TXT = "../os/application.txt"

os.environ.get()
apps = ["init", "terminal", "virtio-blk", "fs", "sdcard"]

with open(OS_APPLICATION_TXT, "w") as f:
    f.write(" ".join(apps))
