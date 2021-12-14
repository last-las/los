import os

OS_APPLICATION_TXT = "../os/application.txt"

base_address = 0x83500000
step = 0x20000
linker = 'linker.ld'
with open(linker, "r") as f:
    origin_content = f.read()

app_id = 0
apps = os.listdir("src/bin/")
apps.sort()

with open(OS_APPLICATION_TXT, "w") as f:
    f.write(" ".join(list(map(lambda x: x.replace(".rs", ""), apps))))

for app in apps:
    app = app[:app.find(".")]
    new_content = origin_content.replace(hex(base_address), hex(base_address + app_id * step))
    with open(linker, "w") as f:
        f.write(new_content)
    os.system("cargo build --release --bin {}".format(app))
    print("[build.py] application {} start with address {}".format(app, hex(base_address + app_id * step)))
    app_id += 1

with open(linker, "w") as f:
    f.write(origin_content)
