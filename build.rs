fn main() {
    println!(r"cargo:rustc-link-search=./sysroot/opt/vc/lib");
    println!(r"cargo:rustc-link-search=./sysroot/opt/vc/include");
    println!(r"cargo:rustc-link-search=./sysroot/usr/lib");
    println!(r"cargo:rustc-link-search=./sysroot/usr/include");
    println!(r"cargo:rustc-link-lib=brcmEGL");
    println!(r"cargo:rustc-link-lib=brcmGLESv2");
}
