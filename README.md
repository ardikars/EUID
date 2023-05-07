# DRAFT#1

## Extendable Universal Unique Identifier

Extendable means you can use this ID for sharding or customizing timestamp epoch. 

### Components

* 3 bit version number
* 4 bit shard_id length (optional, 0 means shard Id is isn't required)
* N bit of shard_id
* 42 bit timestamp with customizable epoch (default is unix epoch)
* 64 + remaining bit (in shard_id block) random number


### Encoding

EUID use bech32m for string representation of the ID.


### Binary layout format (Big Endian)

```textmate
0                   1                   2                   3
 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                       32_bit_uint_t_high                      |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| 10_bit_uint_t_low |   N Bit Random + Shard ID   |SID Len| Ver |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                       32_bit_uint_random                      |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                       32_bit_uint_random                      |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

### Reference implementation

Reference implementation is in src/ directory.

### Inspired by

* [UUID](https://www.ietf.org/rfc/rfc4122.txt)
* [ULID](https://github.com/ulid/spec)
* [Instagram Engineering](https://victoryosayi.medium.com/ulid-universally-unique-lexicographically-sortable-identifier-d75c253bc6a8)