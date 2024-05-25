cargo +stable-i686-pc-windows-msvc build --release
mkdir bin
copy target\release\iorp_core.dll bin\iorp_core.dll