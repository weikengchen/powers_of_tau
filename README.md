# Powers of Tau

This is a fork from [ebfull/powersoftau](https://github.com/ebfull/powersoftau) for the purpose of extracting a SRS suitable for Marlin/Plonk and other variants of KZG, from the Sapling Zcash setup parameters. The parameters are extracted in raw numerical strings. Then, one can use Findora's tool to convert them into the format for arkworks: https://github.com/sunhuachuang/export-setup-parameters

One needs to download the transcript from Internet Archive of the Sapling setup: https://archive.org/details/transcript_201804

It also contains the code of [zkcrypto/pairing](https://github.com/zkcrypto/pairing), with a small modification to expose the x, y, z coordinates.

## Instructions

Once the transcript file is downloaded, one can check its checksum against the one posted on Internet Archive as well as the one in the [CHECKSUM](./CHECKSUM) file. Then, one can run `cargo run` to get the parameters.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
