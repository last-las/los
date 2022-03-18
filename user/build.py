import os

OS_APPLICATION_TXT = "../os/application.txt"

apps = os.listdir("src/bin/")
apps.sort()

with open(OS_APPLICATION_TXT, "w") as f:
    f.write(" ".join(list(map(lambda x: x.replace(".rs", ""), apps))))

os.system("cargo build --bins --release")