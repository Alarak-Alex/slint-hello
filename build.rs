fn main() {
    // 编译.slint文件
    slint_build::compile("ui/app-window.slint").unwrap();

    // // 检查目标平台
    // let target = std::env::var("TARGET").unwrap_or_default();

    // // 编译时默认为Qt后端
    // println!("cargo:rustc-cfg=feature=\"backend-qt\"");
    // println!("cargo:rustc-cfg=feature=\"i-slint-backend-qt\"");
    // println!("cargo:rustc-cfg=feature=\"renderer-software\"");

    // if target == "arm-unknown-linux-gnueabi" {
    //     // 告诉Rust和Cargo我们要使用Qt后端
    //     println!("cargo:rustc-env=SLINT_BACKEND=qt");
    //     println!("cargo:rustc-env=SLINT_FORCE_BACKEND=qt");
    //     println!("cargo:rustc-env=SLINT_NO_QT_AUTODETECT=1");
    //     println!("cargo:rustc-env=SLINT_QT_NO_PLUGIN_SYSTEM=1");
    //     println!("cargo:rustc-env=QT_VERSION=5.12.5");
    //     println!("cargo:rustc-env=SLINT_ENABLE_QT_BACKEND=1");

    //     // 设置Qt库路径
    //     let sysroot = "/Users/nulltech/WorkSpace/arm-buildroot-linux-gnueabi_sdk-buildroot/arm-buildroot-linux-gnueabi/sysroot";
    //     let qt_lib_path = format!("{}/usr/lib", sysroot);
    //     let qt_include_path = format!("{}/usr/include", sysroot);

    //     // 添加链接器搜索路径
    //     println!("cargo:rustc-link-search={}", qt_lib_path);
    //     println!("cargo:rustc-link-search={}/qt5", qt_lib_path);
    //     println!("cargo:rustc-link-arg=-L{}", qt_lib_path);
    //     println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib");

    //     // 明确链接Qt库 - 避免使用dylib前缀，直接链接
    //     println!("cargo:rustc-link-lib=Qt5Widgets");
    //     println!("cargo:rustc-link-lib=Qt5Gui");
    //     println!("cargo:rustc-link-lib=Qt5Core");
    //     println!("cargo:rustc-link-lib=Qt5Network");

    //     // 系统库
    //     println!("cargo:rustc-link-lib=pthread");
    //     println!("cargo:rustc-link-lib=m");
    //     println!("cargo:rustc-link-lib=dl");
    //     println!("cargo:rustc-link-lib=rt");
    //     println!("cargo:rustc-link-lib=c");
    //     println!("cargo:rustc-link-lib=gcc_s");

    //     // 添加链接器选项
    //     println!("cargo:rustc-link-arg=-Wl,--no-as-needed");
    //     println!("cargo:rustc-link-arg=-Wl,--allow-shlib-undefined");

    //     // 添加Qt头文件路径
    //     println!(
    //         "cargo:rustc-env=CPLUS_INCLUDE_PATH={0}:{0}/qt5",
    //         qt_include_path
    //     );
    //     println!("cargo:rustc-link-arg=-I{}", qt_include_path);
    //     println!("cargo:rustc-link-arg=-I{}/qt5", qt_include_path);

    //     // 设置sysroot
    //     println!("cargo:rustc-link-arg=--sysroot={}", sysroot);

    //     // 确保Qt知道在哪里找插件
    //     println!("cargo:rustc-env=QT_PLUGIN_PATH=/usr/lib/qt5/plugins");

    //     // 确保链接时包含Slint的Qt支持
    //     println!("cargo:rustc-env=CARGO_FEATURE_BACKEND_QT=1");
    //     println!("cargo:rustc-env=CARGO_FEATURE_I_SLINT_BACKEND_QT=1");
    //     println!("cargo:rustc-env=CARGO_FEATURE_RENDERER_SOFTWARE=1");

    //     // 显式告诉i-slint-backend-qt要使用哪些环境变量
    //     println!("cargo:rustc-env=CARGO_FEATURE_ENABLE=1");
    //     println!("cargo:rustc-env=RUST_SLINT_WITH_QT=1");
    // } else {
    //     // 本地编译
        println!("cargo:rustc-link-arg=-Wl,-rpath,/Users/nulltech/Qt/6.8.2/macos/lib");
    // }
}
