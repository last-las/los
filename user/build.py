import os

OS_APPLICATION_TXT = "../os/application.txt"

apps = os.listdir("./lib/src/bin/")
apps.append("shell.rs")
apps.append("init.rs")
apps.append("terminal.rs")
apps.append("virtio-blk.rs")
apps.sort()

with open(OS_APPLICATION_TXT, "w") as f:
    f.write(" ".join(list(map(lambda x: x.replace(".rs", ""), apps))))
