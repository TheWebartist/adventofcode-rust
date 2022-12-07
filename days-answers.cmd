@echo off

for /D %%d in ("day *") do (
    cd "%%d"
    rustc isis-milann.rs
    isis-milann.exe
    cd ..
)
