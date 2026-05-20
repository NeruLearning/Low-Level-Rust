@echo off

mkdir ..\build
pushd ..\build

rustc -O ..\Project\Interface.rs -l user32 -C panic=abort -C link-arg="/ENTRY:WinMainCRTStartup" && .\Interface.exe

popd