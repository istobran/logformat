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

## License

[GPLv3](https://opensource.org/licenses/GPL-3.0)

Copyright (c) 2021-present, BangZ