@echo off
setlocal

echo Building rust_i386 image...
docker build -t rust_i386 .

if errorlevel 1 exit /b 1

echo Building project...
docker run --rm ^
  -v "%cd%:/workspace" ^
  rust_i386 ^
  bash -c "cargo build --release --target i686-unknown-linux-gnu"

if errorlevel 1 exit /b 1

mkdir bin 2>nul

copy /Y target\i686-unknown-linux-gnu\release\libiorp_core.so bin\iorp_core.so

echo Done.