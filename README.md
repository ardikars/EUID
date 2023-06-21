# DRAFT#3

## Extendable Universally Unique Identifier

Extendable Universally Unique Identifier or EUID contains two main components header and random number. The header store information about the ID and user-attached data (extension). Timestamps (milliseconds) are also included in the header to make EUID sortable, but the order is not guaranteed if EUID is generated with the same milliseconds. We can provide some guarantee regarding sort order by incrementing random number (64 bits at least significat bits). In case overflow happens when incrementing random number, the generation should fail.

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

The encoding/decoding symbols use a set of 10 digits and 22 letters, excluding 4 of the 26 letters: I L O U.

When decoding, upper and lower case letters are accepted, and i and l will be treated as 1 and o will be treated as 0. When encoding, only upper-case letters are used.

The "check-mod symbols" are added to the last string for detecting transmission and entry errors early and inexpensively.
The "check-mod symbol" encodes the number modulus 127. We can use the remaining bits and add one symbol (2 bits + 5 bits), so our final encoded ID is 27 symbols.


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
| 10 | `A` `a` | `A` |
| 11 | `B` `b` | `B` |
| 12 | `C` `c` | `C` |
| 13 | `D` `d` | `D` |
| 14 | `E` `e` | `E` |
| 15 | `F` `f` | `F` |
| 16 | `G` `g` | `G` |
| 17 | `H` `h` | `H` |
| 18 | `J` `j` | `J` |
| 19 | `K` `k` | `K` |
| 20 | `M` `m` | `M` |
| 21 | `N` `n` | `N` |
| 22 | `P` `p` | `P` |
| 23 | `Q` `q` | `Q` |
| 24 | `R` `r` | `R` |
| 25 | `S` `s` | `S` |
| 26 | `T` `t` | `T` |
| 27 | `V` `v` | `V` |
| 28 | `W` `w` | `W` |
| 29 | `X` `x` | `X` |
| 30 | `Y` `y` | `Y` |
| 31 | `Z` `z` | `Z` |

## Monotonicity

To guarantee a sortable ID, we split 64 bits of randomness (least significant bits) into two parts (high and low). The "high part" is incremented by 1, and the "low part" is a randomly generated number.

### Reference implementation

Reference implementation is in `src` directory.

### Inspired by

* [UUID](https://www.ietf.org/rfc/rfc4122.txt)
* [ULID](https://github.com/ulid/spec)
* [Instagram Engineering](https://instagram-engineering.tumblr.com/post/10853187575/sharding-ids-at-instagram)
* [Crockford's Base32](https://www.crockford.com/base32.html)
