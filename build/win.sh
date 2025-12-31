cargo tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc

cp ./target/x86_64-pc-windows-msvc/release/bundle/nsis/FlavorApp_*_x64-setup.exe ./build/out/FlavorApp.exe
cp ./target/x86_64-pc-windows-msvc/release/bundle/nsis/FlavorApp_*_x64-setup.exe.sig ./build/out/FlavorApp.exe.sig