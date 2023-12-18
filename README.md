# `pe_info`

Small naive utility to detect which MSVC version/Windows version was used to compile an executable.

This utility relies almost entirely on self reported data it is not accurate on executables where the author might want to obfuscate the origin.

# Build

1. Install [rust](https://www.rust-lang.org/tools/install).
2. Clone the repository with `git clone https://github.com/gtker/pe_info`.
3. Run with `cargo run --release -- [OPTIONS] [FILES]`, or build with `cargo build --release` and move the binary from `./target` to your `PATH`.

# Usage

Grab the [newest release](https://github.com/gtker/pe_info/releases).

```
Usage: pe_info [OPTIONS] [FILES]...

Arguments:
  [FILES]...  Files to analyze

Options:
  -q, --quiet    Do not print out intermediate results
  -s, --strict   Return error code for likely invalid information
  -h, --help     Print help
  -V, --version  Print version
```

Usage on some Warcraft 3 executables:

```text
$ pe_info War3_1.13.exe Storm_1.13.dll Game_1.13.dll game.dll Storm.dll War3_1.0.exe
Reading 'War3_1.13.exe'
[ Self Reported ] Timestamp is 2003-05-18 23:44:04 UTC, which means the newest possible version is Visual Studio .NET 2003 (24 days after release)
[ Heuristic     ] Links against msvcrt.dll which suggests compiler is either Visual C++ 4.2 (valid for timestamp), Visual C++ 5.0 (Visual Studio 97) (valid for timestamp), Visual C++ 6.0 (Visual Studio 6.0) (valid for timestamp)
[ Heuristic     ] Imported symbol '??1type_info@@UAE@XZ' from 'msvcrt.dll' suggests C++ was used.
[ Self Reported ] Linker version is 6.0, which suggests linker is Visual C++ 6.0 (Visual Studio 6.0)
[ Self Reported ] Operating system version is 4.0 which suggests Windows 95
[ Self Reported ] Subsystem version 4.0 agrees with operating system version of Windows 95
Compiled: 2003-05-18 23:44:04 UTC
Compiler (one of):
         Visual C++ 4.2
         Visual C++ 5.0 (Visual Studio 97)
         Visual C++ 6.0 (Visual Studio 6.0)
Linker: Visual C++ 6.0 (Visual Studio 6.0)
Operating System: Windows 95

Reading 'Storm_1.13.dll'
[ Self Reported ] Timestamp is 2003-05-18 23:43:38 UTC, which means the newest possible version is Visual Studio .NET 2003 (24 days after release)
[ Heuristic     ] Links against MSVCRT.dll which suggests compiler is either Visual C++ 4.2 (valid for timestamp), Visual C++ 5.0 (Visual Studio 97) (valid for timestamp), Visual C++ 6.0 (Visual Studio 6.0) (valid for timestamp)
[ Heuristic     ] Imported symbol '??1type_info@@UAE@XZ' from 'MSVCRT.dll' suggests C++ was used.
[ Self Reported ] Linker version is 6.0, which suggests linker is Visual C++ 6.0 (Visual Studio 6.0)
[ Self Reported ] Operating system version is 4.0 which suggests Windows 95
[ Self Reported ] Subsystem version 4.0 agrees with operating system version of Windows 95
Compiled: 2003-05-18 23:43:38 UTC
Compiler (one of):
         Visual C++ 4.2
         Visual C++ 5.0 (Visual Studio 97)
         Visual C++ 6.0 (Visual Studio 6.0)
Linker: Visual C++ 6.0 (Visual Studio 6.0)
Operating System: Windows 95

Reading 'Game_1.13.dll'
[ Self Reported ] Timestamp is 2003-05-18 23:57:48 UTC, which means the newest possible version is Visual Studio .NET 2003 (24 days after release)
[ Heuristic     ] Links against MSVCRT.dll which suggests compiler is either Visual C++ 4.2 (valid for timestamp), Visual C++ 5.0 (Visual Studio 97) (valid for timestamp), Visual C++ 6.0 (Visual Studio 6.0) (valid for timestamp)
[ Heuristic     ] Imported symbol '??1type_info@@UAE@XZ' from 'MSVCRT.dll' suggests C++ was used.
[ Self Reported ] Linker version is 6.0, which suggests linker is Visual C++ 6.0 (Visual Studio 6.0)
[ Self Reported ] Operating system version is 4.0 which suggests Windows 95
[ Self Reported ] Subsystem version 4.0 agrees with operating system version of Windows 95
Compiled: 2003-05-18 23:57:48 UTC
Compiler (one of):
         Visual C++ 4.2
         Visual C++ 5.0 (Visual Studio 97)
         Visual C++ 6.0 (Visual Studio 6.0)
Linker: Visual C++ 6.0 (Visual Studio 6.0)
Operating System: Windows 95

Reading 'game.dll'
[ Self Reported ] Timestamp is 2016-12-09 06:05:18 UTC, which means the newest possible version is Visual Studio 2015 (538 days after release)
[ Heuristic     ] Links against MSVCR120.dll which suggests compiler is Visual Studio 2013, which is possible with the release date of the compiler
[ Heuristic     ] Links against MSVCP120.dll which suggests compiler is Visual Studio 2013, which is possible with the release date of the compiler
[ Heuristic     ] Imported symbol '?SCreateProcess@@YIHPBDPAD@Z' from 'Storm.dll' suggests C++ was used.
[ Heuristic     ] Imported symbol '?terminate@@YAXXZ' from 'MSVCR120.dll' suggests C++ was used.
[ Heuristic     ] Imported symbol '?_Syserror_map@std@@YAPBDH@Z' from 'MSVCP120.dll' suggests C++ was used.
[ Heuristic     ] Import of symbol 'strtoull' from 'MSVCR120.dll' suggests at least standard C++11
[ Heuristic     ] Import of symbol 'strtoll' from 'MSVCR120.dll' suggests at least standard C++11
[ Self Reported ] Linker version is 12.0, which suggests linker is Visual Studio 2013
[ Self Reported ] Operating system version is 5.1 which suggests Windows XP
[ Self Reported ] Subsystem version 5.1 agrees with operating system version of Windows XP
Compiled: 2016-12-09 06:05:18 UTC
Compiler: Visual Studio 2013
Linker: Visual Studio 2013
C++ Standard used: At least C++11
Operating System: Windows XP

Reading 'Storm.dll'
[ Self Reported ] Timestamp is 2016-12-09 06:05:51 UTC, which means the newest possible version is Visual Studio 2015 (538 days after release)
[ Heuristic     ] Links against MSVCR120.dll which suggests compiler is Visual Studio 2013, which is possible with the release date of the compiler
[ Heuristic     ] Imported symbol '?terminate@@YAXXZ' from 'MSVCR120.dll' suggests C++ was used.
[ Self Reported ] Linker version is 12.0, which suggests linker is Visual Studio 2013
[ Self Reported ] Operating system version is 5.1 which suggests Windows XP
[ Self Reported ] Subsystem version 5.1 agrees with operating system version of Windows XP
Compiled: 2016-12-09 06:05:51 UTC
Compiler: Visual Studio 2013
Linker: Visual Studio 2013
Operating System: Windows XP

Reading 'War3_1.0.exe'
[ Self Reported ] Timestamp is 2002-06-06 11:25:18 UTC, which means the newest possible version is Visual Studio .NET 2002 (113 days after release)
[ Heuristic     ] Links against msvcrt.dll which suggests compiler is either Visual C++ 4.2 (valid for timestamp), Visual C++ 5.0 (Visual Studio 97) (valid for timestamp), Visual C++ 6.0 (Visual Studio 6.0) (valid for timestamp)
[ Heuristic     ] Imported symbol '??1type_info@@UAE@XZ' from 'msvcrt.dll' suggests C++ was used.
[ Self Reported ] Linker version is 6.0, which suggests linker is Visual C++ 6.0 (Visual Studio 6.0)
[ Self Reported ] Operating system version is 4.0 which suggests Windows 95
[ Self Reported ] Subsystem version 4.0 agrees with operating system version of Windows 95
Compiled: 2002-06-06 11:25:18 UTC
Compiler (one of):
         Visual C++ 4.2
         Visual C++ 5.0 (Visual Studio 97)
         Visual C++ 6.0 (Visual Studio 6.0)
Linker: Visual C++ 6.0 (Visual Studio 6.0)
Operating System: Windows 95
```

# Linker Versions

[source](https://web.archive.org/web/20231216085511/https://mariusbancila.ro/blog/2015/08/12/version-history-of-vc-mfc-and-atl/)

[wiki source under `Internal version numbering`](https://web.archive.org/web/20231216085732/https://en.wikipedia.org/wiki/Microsoft_Visual_C%2B%2B)

# Dll Versions

[source](https://web.archive.org/web/20231216075910/http://zuga.net/articles/vs-crt-runtime-library-versions/)

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

