# logformat
A easy command-line tool to format logging output of another process with specified date-time format.

## Example Usage
```bash
echo "Example Usage" | logformat

# output:
# 2021-10-11 23:02:05 Example Usage
```
You can customize the date-time format by pass string param into program
```bash
echo "Example Usage" | logformat "[Tag] %H:%M:%S %z"

# output:
# [Tag] 23:03:02 +0800 Example Usage
```
Note: The available specifiers are following [strftime](https://docs.rs/chrono/0.4/chrono/format/strftime/index.html#specifiers)  

And it also support processing consistent stream:
```bash
ping 127.0.0.1 | logformat "[Example] %H:%M:%S"

# output:
# [Example] 23:14:16 PING 127.0.0.1 (127.0.0.1): 56 data bytes
# [Example] 23:14:16 64 bytes from 127.0.0.1: icmp_seq=0 ttl=64 time=0.074 ms
# [Example] 23:14:17 64 bytes from 127.0.0.1: icmp_seq=1 ttl=64 time=0.072 ms
# [Example] 23:14:18 64 bytes from 127.0.0.1: icmp_seq=2 ttl=64 time=0.105 ms
# [Example] 23:14:19 64 bytes from 127.0.0.1: icmp_seq=3 ttl=64 time=0.100 ms
# ...
```
Appending postfix:
```bash
ping 127.0.0.1 | logformat "__BODY__ DONE."

# output:
# PING 127.0.0.1 (127.0.0.1): 56 data bytes DONE.
# 64 bytes from 127.0.0.1: icmp_seq=0 ttl=64 time=0.074 ms DONE.
# 64 bytes from 127.0.0.1: icmp_seq=1 ttl=64 time=0.072 ms DONE.
# 64 bytes from 127.0.0.1: icmp_seq=2 ttl=64 time=0.105 ms DONE.
# 64 bytes from 127.0.0.1: icmp_seq=3 ttl=64 time=0.100 ms DONE.
# ...
```
The `__BODY__` specifier means a line of original log body which is reading from stdin, you can inject this to any position of formatting pattern for flexible usage.

Using prefix and postfix at the same time:
```bash
ping 127.0.0.1 | logformat "[Example] %H:%M:%S __BODY__ DONE."

# output:
# [Example] 23:14:16 PING 127.0.0.1 (127.0.0.1): 56 data bytes DONE.
# [Example] 23:14:16 64 bytes from 127.0.0.1: icmp_seq=0 ttl=64 time=0.074 ms DONE.
# [Example] 23:14:17 64 bytes from 127.0.0.1: icmp_seq=1 ttl=64 time=0.072 ms DONE.
# [Example] 23:14:18 64 bytes from 127.0.0.1: icmp_seq=2 ttl=64 time=0.105 ms DONE.
# [Example] 23:14:19 64 bytes from 127.0.0.1: icmp_seq=3 ttl=64 time=0.100 ms DONE.
# ...
```

## Installation
Intel x86_64 based pre-compiled Binary has been provided to download directly.

**Linux**:
```bash
# with curl
curl -o /usr/local/bin/logformat -L https://github.com/istobran/logformat/releases/download/v0.1.0/logformat-linux-x86_64 && chmod +x /usr/local/bin/logformat
# or with wget
wget -O /usr/local/bin/logformat https://github.com/istobran/logformat/releases/download/v0.1.0/logformat-linux-x86_64 && chmod +x /usr/local/bin/logformat

# uninstall
rm -f /usr/local/bin/logformat
```

**macOS**:
```bash
curl -o /usr/local/bin/logformat -L https://github.com/istobran/logformat/releases/download/v0.1.0/logformat-macOS-x86_64 && chmod +x /usr/local/bin/logformat

# uninstall
rm -f /usr/local/bin/logformat
```

**Windows**:
1. Go [Release](https://github.com/istobran/logformat/releases) page and download `logformat-windows-x86_64.exe`
2. rename `logformat-windows-x86_64.exe` to `logformat.exe`
3. move to windows System32 directory `C:\Windows\System32`

Uninstall: just remove `C:\Windows\System32\logformat.exe`

## Compile from source
Compilation require rust to be installed first, see [detail](https://www.rust-lang.org/learn/get-started).  
About rust version:
```bash
> rustc --version
rustc 1.55.0 (c8dfcfe04 2021-09-06)
> cargo --version
cargo 1.55.0 (32da73ab1 2021-08-23)
```
Then clone repo into a local directory
```bash
> git clone git@github.com:istobran/logformat.git
```
Build operations now are available to run:
```bash
> cargo run --release
```
Binary executable file will be generated to `target/release/logformat`
```bash
> file target/release/logformat
target/release/logformat: Mach-O 64-bit executable x86_64
```

## License

[GPLv3](https://opensource.org/licenses/GPL-3.0)

Copyright (c) 2021-present, BangZ