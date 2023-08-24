import os
import shutil

def main():
    if os.path.isdir("./packup"):
        print("packup exist: passing")
    else:
        os.mkdir("./packup")
        os.mkdir("./packup/tmp")

    if os.path.isfile("./packup/tmp/7z.exe"):
        print("7zip exist: passing")
        pass
    else:
        print("Geting a copy of 7zip...")
        os.system("curl https://www.7-zip.org/a/7zr.exe --output ./packup/tmp/7z.exe")
    
    os.system("cargo b --bin app -r")

    if os.name in ["posix", "linux", "linux2"]:
        packup = "./packup/linux/"
        if os.path.isdir(packup):
            print(packup + "exist: passing")
        else:
            os.mkdir(packup)
            shutil.copyfile("./target/release/app", packup)
            #shutil.copyfile("./target/release/install", packup)
    elif os.name in ["nt", "Windows", "win32"]:
        packup = "./packup/Windows/"
        if os.path.isdir(packup):
            print(packup + "exist: passing")
        else:
            os.mkdir(packup)
            shutil.copyfile("./target/release/app.exe", packup + "app.exe")
            #shutil.copyfile("./target/release/install.exe", packup + "install.exe")

    os.system("\"./packup/tmp/7z.exe\" a " + packup)
main()
    