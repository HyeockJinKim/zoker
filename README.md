# Zoker
> The zoker is a tool you can use to apply `zero knowledge proofs` to smart contracts.

![CI](https://github.com/HyeockJinKim/zoker-parser/workflows/CI/badge.svg)

`Zoker` is a tool for parsing grammars using the [lalrpop][lalrpop] parser
tool to generate zero knowledge proof circuits.

When you write a DSL of the proposed solidity subset, the `Zoker` preprocesses 
the DSL to produce a proof of zero knowledge proof and a verification smart contract
to verify the proof.

See [zoker.lalrpop][zoker.lalrpop] for the detailed syntax of the DSL.

## Documentation

## How to use

## Example Uses

## License

This project is licensed under the MIT license. Please see the [LICENSE][LICENSE] file for more details.

[lalrpop]: https://github.com/lalrpop/lalrpop 
[LICENSE]: LICENSE
[zoker.lalrpop]: src/zoker_parser.lalrpop