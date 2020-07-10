<!-- cargo-sync-readme start -->

# rarity-mention

`rarity-mention` is a utility crate for the Discord [`twilight-rs`]
ecosystem to mention its model types.

With this library, you can create mentions for various types, such as users,
emojis, roles, members, or channels.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rarity-mention = { branch = "main", git = "https://github.com/rarity-rs/mention" }
```

## Examples

Create a mention formatter a user ID, and then format it in a message:

```rust
use rarity_mention::Mention;
use twilight_model::id::UserId;

let user_id = UserId(123);
let message = format!("Hey there, {}!", user_id.mention());
```

[`twilight-rs`]: https://github.com/twilight-rs/twilight

<!-- cargo-sync-readme end -->
