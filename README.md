# fyyd-api

[![Built with Nix](https://img.shields.io/static/v1?label=built%20with&message=nix&color=5277C3&logo=nixos&style=flat-square&logoColor=ffffff)](https://builtwithnix.org)
[![Crates](https://img.shields.io/crates/v/fyyd-api?style=flat-square)](https://crates.io/crates/fyyd-api)
[![Documentation](https://docs.rs/fyyd-api/badge.svg)](https://docs.rs/fyyd-api)

Rust bindings to the fyyd api.
Currently the api version 2 is supported.



# Limitations

The library currently fits my needs, but is still limited, notably:

- Not all endpoints are supported
- It is fully async and doesn't support blocking calls
- It uses reqwest as an http backend

If you have different needs, or find improvements, 
I am always happy about contributions.
Please see the contributing section for that.


# Contributing
[How to contribute.](./docs/CONTRIBUTING.md)

# Reference
- [Fyyd](https://fyyd.de/)
- [Fyyd Api Documentation](https://github.com/eazyliving/fyyd-api)

# License
MIT
