import os

OS_APPLICATION_TXT = "../os/application.txt"

board = os.environ.get("BOARD")
if board == "qemu":
    apps = ["init", "terminal", "virtio-blk", "fs"]
elif board == "k210":
    apps = ["init", "terminal", "sdcard", "fs", "idle"]
else:
    exit(1)

with open(OS_APPLICATION_TXT, "w") as f:
    f.write(" ".join(apps))
