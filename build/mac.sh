cargo tauri build --bundles app,dmg

cp ./target/release/bundle/dmg/FlavorApp_*_aarch64.dmg ./build/out/FlavorApp.dmg
cp ./target/release/bundle/macos/FlavorApp.app.tar.gz ./build/out/FlavorApp.app.tar.gz
cp ./target/release/bundle/macos/FlavorApp.app.tar.gz.sig ./build/out/FlavorApp.app.tar.gz.sig