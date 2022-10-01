# Find Offset Containing Strings (of bytes) in a VM

You just have to specify a PID of quemu process, RAM size assiciated with a VM and a pattern to scan :)


## HELP
[root@rapier-v focs_vm]# ./target/release/focs_vm -h

focs_vm 0.1.0

Kamil Stawiarski <kamil@ora-600.pl>

Tool for finding patterns in memory



USAGE:

    focs_vm [OPTIONS] --memory-size <MEMORY_SIZE> --pid <PID> --pattern <PATTERN>



OPTIONS:

    -b, --buffer <BUFFER>              Size of a buffer to print [default: 256]

    -h, --help                         Print help information

    -m, --memory-size <MEMORY_SIZE>    Size of memory segment to scan

    -p, --pid <PID>                    PID of the process to scan

    -p, --pattern <PATTERN>            Pattern in hex to search for

    -P, --parallel <PARALLEL>          Parallel degree [default: 4]

    -V, --version                      Print version information


## Sample usage: 
```bash
[root@rapier-v focs_vm]# ./target/release/focs_vm --pid 10922 --memory-size $((6291456*1024)) --pattern '72 6f 6f 74 3a 24 36 24' -P 4
Found map at the start offset = 140324162764800 	 end offset = 140330605215744

Scanning memory from 140324162764800 to 140325773377536 in a separate thread
Scanning memory from 140325773377536 to 140327383990272 in a separate thread
Scanning memory from 140328994603008 to 140330605215744 in a separate thread
Scanning memory from 140327383990272 to 140328994603008 in a separate thread
Scanned: 6 %
Position of pattern found at 140324287193088
Length: 256 (0x100) bytes
0000:   72 6f 6f 74  3a 24 36 24  75 78 6b 51  43 42 79 4d   root:$6$uxkQCByM
0010:   62 50 66 35  30 74 73 35  24 72 64 67  73 46 57 47   bPf50ts5$rdgsFWG
0020:   4f 57 66 57  77 75 6f 62  2e 58 32 6d  7a 6b 35 55   OWfWwuob.X2mzk5U
0030:   42 6b 51 68  46 63 47 56  63 59 44 75  55 71 62 72   BkQhFcGVcYDuUqbr
0040:   79 33 5a 33  42 70 36 59  67 32 4b 75  50 77 56 61   y3Z3Bp6Yg2KuPwVa
0050:   36 43 61 65  4d 74 61 48  75 6e 4c 54  6c 6d 4e 61   6CaeMtaHunLTlmNa
0060:   71 6e 76 34  76 35 46 4b  53 48 54 54  33 32 31 3a   qnv4v5FKSHTT321:
0070:   31 39 32 36  31 3a 30 3a  39 39 39 39  39 3a 37 3a   19261:0:99999:7:
0080:   3a 3a 0a 62  69 6e 3a 2a  3a 31 38 33  30 37 3a 30   ::.bin:*:18307:0
0090:   3a 39 39 39  39 39 3a 37  3a 3a 3a 0a  64 61 65 6d   :99999:7:::.daem
00a0:   6f 6e 3a 2a  3a 31 38 33  30 37 3a 30  3a 39 39 39   on:*:18307:0:999
00b0:   39 39 3a 37  3a 3a 3a 0a  61 64 6d 3a  2a 3a 31 38   99:7:::.adm:*:18
00c0:   33 30 37 3a  30 3a 39 39  39 39 39 3a  37 3a 3a 3a   307:0:99999:7:::
00d0:   0a 6c 70 3a  2a 3a 31 38  33 30 37 3a  30 3a 39 39   .lp:*:18307:0:99
00e0:   39 39 39 3a  37 3a 3a 3a  0a 73 79 6e  63 3a 2a 3a   999:7:::.sync:*:
00f0:   31 38 33 30  37 3a 30 3a  39 39 39 39  39 3a 37 3a   18307:0:99999:7:

Scanned: 100 %
```
