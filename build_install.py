import os
import shutil

print()
print("Warning: this program is for Building install file for app NOT app it self.")
print()

def main():
    if os.path.isdir("./packup"):
        print("packup exist: passing")
    else:
        os.mkdir("./packup")
    
    os.system("cargo b --bin app --locked --release")

    packup = "./packup/" + os.name + '/'

    if os.path.isdir(packup):
        print(packup + " exist: passing")
    else:
        os.mkdir(packup)
        os.mkdir(packup + "lib")
    
    if os.name in ["posix", "linux", "linux2"]:
        appexedir = "./target/release/app"
        appexe = "./app"
        installexedir = "./install/target/release/install"
        installexe = "./install-" + os.name
    elif os.name in ["nt", "Windows", "win32"]:
        appexedir = "./target/release/app.exe"
        appexe = "./app.exe"
        installexedir = "./install/target/release/install.exe"
        installexe = "./install-" + os.name + ".exe"

    for dir in ["lib/frontend", "lib/noVNC", "lib/setup"]:
        if os.path.isdir(packup + dir):
            print(packup + dir + " exist: passing")
        else:
            shutil.copytree("./" + dir, packup + dir)

    index = open("./packup/" + "index.txt", "w")
    index.write("." + packup)
    index.close()

    shutil.copyfile(appexedir, packup + appexe)
    os.system("cargo build --manifest-path=install/Cargo.toml --release")
    shutil.copyfile(installexedir, installexe)
    print("Finished: Run '" + installexe + "' to install.")
main()
    