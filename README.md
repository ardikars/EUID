# DRAFT#5

## Extendable Universally Unique Identifier

Binary layout (Big Endian):
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

### Reference implementation

Reference implementation is in `src` directory.

### Inspired by

* [UUID](https://www.ietf.org/rfc/rfc4122.txt)
* [ULID](https://github.com/ulid/spec)
* [Instagram Engineering](https://instagram-engineering.tumblr.com/post/10853187575/sharding-ids-at-instagram)
* [Crockford's Base32](https://www.crockford.com/base32.html)
