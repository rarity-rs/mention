//! # rarity-mention
//!
//! `rarity-mention` is a utility crate for the Discord [`twilight-rs`]
//! ecosystem to mention its model types.
//!
//! With this library, you can create mentions for various types, such as users,
//! emojis, roles, members, or channels.
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rarity-mention = { branch = "main", git = "https://github.com/rarity-rs/mention" }
//! ```
//!
//! ## Examples
//!
//! Create a mention formatter a user ID, and then format it in a message:
//!
//! ```rust
//! use rarity_mention::Mention;
//! use twilight_model::id::UserId;
//!
//! let user_id = UserId(123);
//! let message = format!("Hey there, {}!", user_id.mention());
//! ```
//!
//! [`twilight-rs`]: https://github.com/twilight-rs/twilight

#![doc(html_logo_url = "https://raw.githubusercontent.com/rarity-rs/assets/main/logo.png")]
#![deny(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    unsafe_code,
    unused,
    warnings
)]

use std::fmt::{Display, Formatter, Result as FmtResult};
use twilight_model::{
    channel::{
        CategoryChannel, Channel, Group, GuildChannel, PrivateChannel, TextChannel, VoiceChannel,
    },
    guild::{Emoji, Member},
    id::{ChannelId, EmojiId, RoleId, UserId},
    user::{CurrentUser, User},
};

/// Formatter to mention a resource that implements `std::fmt::Display`.
///
/// # Examples
///
/// Mention a `UserId`:
///
/// ```rust
/// use rarity_mention::Mention;
/// use twilight_model::id::UserId;
///
/// assert_eq!("<@123>", UserId(123).mention().to_string());
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MentionFormat<T>(T);

/// Mention a channel. This will format as `<#ID>`.
impl Display for MentionFormat<ChannelId> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_fmt(format_args!("<#{}>", self.0))
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Display for MentionFormat<EmojiId> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_fmt(format_args!("<:emoji:{}>", self.0))
    }
}

/// Mention a role. This will format as `<@&ID>`.
impl Display for MentionFormat<RoleId> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_fmt(format_args!("<@&{}>", self.0))
    }
}

/// Mention a user. This will format as `<@ID>`.
impl Display for MentionFormat<UserId> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_fmt(format_args!("<@{}>", self.0))
    }
}

/// Mention a resource, such as an emoji or user.
///
/// This will create a mention that will link to a user if it exists.
///
/// Look at the implementations list to see what you can mention.
///
/// # Examples
///
/// Mention a `ChannelId`:
///
/// ```rust
/// use rarity_mention::Mention;
/// use twilight_model::id::ChannelId;
///
/// assert_eq!("<#123>", ChannelId(123).mention().to_string());
/// ```
pub trait Mention<T> {
    /// Mention a resource by using its ID.
    fn mention(&self) -> MentionFormat<T>;
}

/// Mention a channel ID. This will format as `<#ID>`.
impl Mention<ChannelId> for ChannelId {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(*self)
    }
}

/// Mention a channel ID. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ ChannelId {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

/// Mention a guild category channel. This will format as `<#ID>`.
impl Mention<ChannelId> for CategoryChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

/// Mention a guild category channel. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ CategoryChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

/// Mention a channel. This will format as `<#ID>`.
impl Mention<ChannelId> for Channel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(match self {
            Self::Group(group) => group.id,
            Self::Guild(guild) => guild_channel_id(guild),
            Self::Private(private) => private.id,
        })
    }
}

/// Mention a channel. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ Channel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

/// Mention the current user. This will format as `<@ID>`.
impl Mention<UserId> for CurrentUser {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(self.id)
    }
}

/// Mention the current user. This will format as `<@ID>`.
impl Mention<UserId> for &'_ CurrentUser {
    fn mention(&self) -> MentionFormat<UserId> {
        (*self).mention()
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Mention<EmojiId> for EmojiId {
    fn mention(&self) -> MentionFormat<EmojiId> {
        MentionFormat(*self)
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Mention<EmojiId> for &'_ EmojiId {
    fn mention(&self) -> MentionFormat<EmojiId> {
        (*self).mention()
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Mention<EmojiId> for Emoji {
    fn mention(&self) -> MentionFormat<EmojiId> {
        MentionFormat(self.id)
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Mention<EmojiId> for &'_ Emoji {
    fn mention(&self) -> MentionFormat<EmojiId> {
        (*self).mention()
    }
}

/// Mention a group. This will format as `<#ID>`.
impl Mention<ChannelId> for Group {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

/// Mention a group. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ Group {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

/// Mention a guild channel. This will format as `<#ID>`.
impl Mention<ChannelId> for GuildChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(guild_channel_id(self))
    }
}

/// Mention a guild channel. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ GuildChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

/// Mention a member's user. This will format as `<@ID>`.
impl Mention<UserId> for Member {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(self.user.id)
    }
}

/// Mention a member's user. This will format as `<@ID>`.
impl Mention<UserId> for &'_ Member {
    fn mention(&self) -> MentionFormat<UserId> {
        (*self).mention()
    }
}

/// Mention a private channel. This will format as `<#ID>`.
impl Mention<ChannelId> for PrivateChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

/// Mention a private channel. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ PrivateChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

/// Mention a role ID. This will format as `<@&ID>`.
impl Mention<RoleId> for RoleId {
    fn mention(&self) -> MentionFormat<RoleId> {
        MentionFormat(*self)
    }
}

/// Mention a role ID. This will format as `<@&ID>`.
impl Mention<RoleId> for &'_ RoleId {
    fn mention(&self) -> MentionFormat<RoleId> {
        (*self).mention()
    }
}

/// Mention a guild text channel. This will format as `<#ID>`.
impl Mention<ChannelId> for TextChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

/// Mention a guild text channel. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ TextChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

/// Mention a user ID. This will format as `<&ID>`.
impl Mention<UserId> for UserId {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(*self)
    }
}

/// Mention a user ID. This will format as `<&ID>`.
impl Mention<UserId> for &'_ UserId {
    fn mention(&self) -> MentionFormat<UserId> {
        (*self).mention()
    }
}

/// Mention a user. This will format as `<&ID>`.
impl Mention<UserId> for User {
    fn mention(&self) -> MentionFormat<UserId> {
        MentionFormat(self.id)
    }
}

/// Mention a user. This will format as `<&ID>`.
impl Mention<UserId> for &'_ User {
    fn mention(&self) -> MentionFormat<UserId> {
        (*self).mention()
    }
}

/// Mention a guild voice channel. This will format as `<#ID>`.
impl Mention<ChannelId> for VoiceChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        MentionFormat(self.id)
    }
}

/// Mention a guild voice channel. This will format as `<#ID>`.
impl Mention<ChannelId> for &'_ VoiceChannel {
    fn mention(&self) -> MentionFormat<ChannelId> {
        (*self).mention()
    }
}

fn guild_channel_id(channel: &GuildChannel) -> ChannelId {
    match channel {
        GuildChannel::Category(c) => c.id,
        GuildChannel::Text(c) => c.id,
        GuildChannel::Voice(c) => c.id,
    }
}

#[cfg(test)]
mod tests {
    use super::Mention;
    use twilight_model::id::{ChannelId, EmojiId, RoleId, UserId};

    #[test]
    fn test_mention_format_channel_id() {
        assert_eq!("<#123>", ChannelId(123).mention().to_string());
    }

    #[test]
    fn test_mention_format_emoji_id() {
        assert_eq!("<:emoji:123>", EmojiId(123).mention().to_string());
    }

    #[test]
    fn test_mention_format_role_id() {
        assert_eq!("<@&123>", RoleId(123).mention().to_string());
    }

    #[test]
    fn test_mention_format_user_id() {
        assert_eq!("<@123>", UserId(123).mention().to_string());
    }
}
