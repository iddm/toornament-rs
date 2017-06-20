# Introduction

## Description

[**Toornament**](https://toornament.com) - is the eSports platform for tournaments organizers, 
their participants and their fans.

[**toornament-rs**](https://github.com/vityafx/toornament-rs) - is a library which implements the 
REST-API of toornament.com. This library fully implements the endpoints stack so you can be sure 
that you can do any kind of things with it. The library is 
[`MIT`-licensed](https://github.com/vityafx/toornament-rs/blob/master/LICENSE) and
[very-well documented](https://docs.rs/toornament/). It has no `unsafe`-blocks of code, uses 
cutting-edge crates as dependencies and provides the interface as simple as possible.

The library provides additionally provides **iter**-like inteface rather than REST-one. It
abstracts code writers from rest-inteface at all, lets them work with `Toornament` entities
directly without calling API methods, increases code readability and writability, and eventually
boosts developing.

The library is thread-safe, the `Toornament` structure implements `Send` and `Sync` traits and
has no mutable methods.
