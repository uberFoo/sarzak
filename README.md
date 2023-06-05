# Sarzak Tool Suite

[![coverage report](https://git.uberfoo.com/sarzak/sarzak/badges/develop/coverage.svg)](https://git.uberfoo.com/sarzak/sarzak/-/commits/develop)

Sarzak is a collection of tools, and perhaps most importantly, models.

## Dwarf

These are notes on the dwarf language, as it evolves.
Thus far, it's very rust-like.

Here is a list of "stuff":
 * no enums
 * no traits
 * struct expression fields don't support shorthand

 `sarzak gen -m v2/lu_dog grace dwarf -s target/sarzak/merlin/lu_dog.json`

 `RUST_LOG=trace dwarfc target/sarzak/lu_dog/lu_dog.tao --sarzak models/sarzak.json`
