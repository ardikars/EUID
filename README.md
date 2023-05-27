# DRAFT#2

## Extendable Universally Unique Identifier

Extendable Universally Unique Identifier or EUID contains two main components header and random number. The header store information about the ID and user-attached data (extension). Timestamps are also included in the header to make EUID sortable, but the order is not guaranteed if EUID is generated with the same milliseconds. We can provide some guarantee regarding sort order by incrementing random number (at least significant bit) by 1. In case overflow happens when incrementing random number, the generation should fail.

Binary layout (Big Endian):
```text
        0               1               2               3
 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |                         Timestamp High                        |
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |   Timestamp Low   |   N Bit Random + Ext Data   |Ext Len| Ver |
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |                             Random                            |
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |                             Random                            |
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```


## Extension

Extension is 15 bits of user-attached data (0-32767). If the data is less than 15 bits then the remaining bits are filled with random data.


## Encoding/Decoding Symbols

The encoding/decoding symbols are using Crockford's Base32 excluding the check symbol (`*`,`~`,`$`,`=`,`U`,`u`) and hyphens (`-`) separator.

| Symbol Value | Decode Symbol | Encode Symbol |
|--------------|---------------|---------------|
| 0 | `0` `O` `o` | `0` |
| 1 | `1` `I` `i` `L` `l` | `1` |
| 2 | `2` | `2` |
| 3 | `3` | `3` |
| 4 | `4` | `4` |
| 5 | `5` | `5` |
| 6 | `6` | `6` |
| 7 | `7` | `7` |
| 8 | `8` | `8` |
| 9 | `9` | `9` |
| 10 | `A` `a` | `A` `a` |
| 11 | `B` `b` | `B` `b` |
| 12 | `C` `c` | `C` `c` |
| 13 | `D` `d` | `D` `d` |
| 14 | `E` `e` | `E` `e` |
| 15 | `F` `f` | `F` `f` |
| 16 | `G` `g` | `G` `g` |
| 17 | `H`  `h`| `H` `h` |
| 18 | `J` `j` | `J` `j` |
| 19 | `K` `k` | `K` `k` |
| 20 | `M` `m` | `M` `m` |
| 21 | `N` `n` | `N` `n` |
| 22 | `P` `p` | `P` `p` |
| 23 | `Q` `q` | `Q` `q` |
| 24 | `R` `r` | `R` `r` |
| 25 | `S` `s` | `S` `s` |
| 26 | `T` `t` | `T` `t` |
| 27 | `V` `v` | `V` `v` |
| 28 | `W` `w` | `W` `w` |
| 29 | `X` `x` | `X` `x` |
| 30 | `Y` `y` | `Y` `y` |
| 31 | `Z` `z` | `Z` `z` |

### Reference implementation

Reference implementation is in `src` directory.

### Inspired by

* [UUID](https://www.ietf.org/rfc/rfc4122.txt)
* [ULID](https://github.com/ulid/spec)
* [Instagram Engineering](https://instagram-engineering.tumblr.com/post/10853187575/sharding-ids-at-instagram)
