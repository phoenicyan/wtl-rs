wtl-rs
=====
Windows WTL GUI library for Rust
This is copy of github.com/varding/wtl-rs with compilation errors fixed for the latest Rust.

Installation
=====
Clone project use commands below:
```
git clone https://github.com/phoenicyan/wtl-rs
cd wtl-rs\pkg
git clone https://github.com/retep998/winapi-rs.git winapi
cd ..
```

Example
=====
There is a simple example in examples/GuidGen directory

Use commands below to build and run:
```
cd .\examples\GuidGen
cargo build
.\target\debug\GuidGen.exe
```

Resource
=====
* http://www.codeproject.com/Articles/3841/WTL-for-MFC-Programmers-Part-I-ATL-GUI-Classes
* http://www.codeproject.com/Articles/3867/WTL-for-MFC-Programmers-Part-II-WTL-GUI-Base-Class
* http://www.codeproject.com/Articles/3948/WTL-for-MFC-Programmers-Part-III-Toolbars-and-Stat
* http://www.codeproject.com/Articles/4028/WTL-for-MFC-Programmers-Part-IV-Dialogs-and-Contro
* http://www.codeproject.com/Articles/4029/WTL-for-MFC-Programmers-Part-V-Advanced-Dialog-UI
