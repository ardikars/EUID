# DRAFT#5

## Extendable Universally Unique Identifier

EUID (Extendable Universally Unique Identifier) is a unique identifier designed to ensure efficient data management through improved sortability and extensibility. EUIDs include timestamps with milliseconds precision and utilize randomness (64 bits, least significant bits) to achieve a predictable order. The extension feature enables the attachment of optional data, enhancing the versatility of EUIDs.

### Timestamps and Randomness

EUIDs include Unix timestamps with precision down to the millisecond level (applicable until the year 3084 AD) to facilitate sortability. However, when generated with the same milliseconds, their order is not guaranteed. To address this, 64 bits of randomness are split into two parts: the "high" which is incremented by 1, and the "low" which receives randomly generated data. This process ensures a predictable sorting order.

### The Extension

EUIDs incorporate an extension feature that allows for the attachment of 15 bits of data (ranging from 0 to 32767). If the attached data occupies less than 15 bits, the remaining bits are filled with randomly generated data. The extension is optional, and when no data needs to be attached, the extension length is set to 0. However, if data attachment is required, the extension length must match the size of the appended data in bits.

### Encoding, Decoding and Check-Mod Symbols for Error Detection

EUIDs use a set of 10 digits and 22 letters for encoding and decoding, excluding four letters (I, L, O, U) from the set of 26. During decoding, EUIDs accept both uppercase and lowercase letters, treating 'i' and 'l' as 1, and 'o' as 0. However, during encoding, only uppercase letters are utilized to ensure consistency. To detect transmission and entry errors inexpensively, check-mod symbols are added to the last string of EUIDs. These symbols encode the number modulus 127, allowing for early error detection. By utilizing the remaining bits and adding one symbol (2 bits + 5 bits), the final encoded ID becomes a 27-character string. A check-mod value of 127 eliminates the need for check-mod validation during decoding.

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


### Binary layout (Big Endian):

```text
        0               1               2               3
  0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |                         Timestamp High                        |
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |      Timestamp Low      | N Bit Random + Ops Ext Data |Ext Len|
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |                             Random                            |
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |                             Random                            |
 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

### Key Attributes of EUIDs

* Compactness: EUIDs are designed with a 128-bit format, optimizing space utilization.
* Human Readability and URL Safety: EUIDs are intentionally crafted for human readability and are secure for use in URLs.
* Lexicographic Sortability: EUIDs facilitate lexicographic sorting, with the option to support monotonically increasing IDs when generated within the same millisecond.
* Extendability: EUIDs provide the capability to append up to 15 bits of user-defined data, enhancing their versatility.
* Canonical Encoding: EUIDs follow a canonical encoding format, resulting in a standardized 27-character string representation.
* Case Insensitivity: EUID decoding is case-insensitive, ensuring consistent interpretation.
* Error Detection: EUIDs utilize a Check-Mod mechanism to detect typographical and transmission errors.

### Reference implementation

Reference implementation is in `src` directory.

### References

* [UUID](https://www.ietf.org/rfc/rfc4122.txt)
* [ULID](https://github.com/ulid/spec)
* [Instagram Engineering](https://instagram-engineering.tumblr.com/post/10853187575/sharding-ids-at-instagram)
* [Crockford's Base32](https://www.crockford.com/base32.html)
