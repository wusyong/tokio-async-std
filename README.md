# tokio-async-std

[![][crates-badge]][crates-url] ![][license-badge] ![][build-bade]

[crates-badge]: https://img.shields.io/crates/v/tokio-async-std
[crates-url]: https://crates.io/crates/tokio-async-std
[license-badge]: https://img.shields.io/crates/l/tokio-async-std
[build-bade]: https://img.shields.io/github/workflow/status/wusyong/tokio-async-std/Main
![](https://imgur.com/TuK06ah.png)

Welp, it's an async runtime that provides exact same `async-std` APIs in the front with `tokio` task executor in the back.

## What does it have?

Above has said it all. This is a fork of `async-std` but replace the executor with `tokio`'s. Everything is working and feels almost exactly the same, but there are still some differences. Basically when it comes to task spawning and runtime configuration, it will behave as how `tokio` presents. So beware on that. Otherwise, it does feels like a drop-in async runtime can use for.

## Is it worth it?

Someone asked for it, so here it is. But frankly speaking, both `tokio` and `async-std` already have comprehensive functionalities. Just choose the one you are fond of shouldn't get anything wrong. This is the one for people who **really really really** want to use interface of `async-std` but also need to use crates from `tokio` ecosystem.

## How do I use it?

Add `tokio-async-std` to `Cargo.toml` as dependency. The main and minor version follow the same semver from `async-std`. Then simply put import it and everything else should feel like the same. The library name is also same.

```rust
use async_std;
```
 