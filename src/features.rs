//! Canonical feature tokens declared by Counter-Strike 1.6 game logic.

use goldsrc_api::FeatureToken;

/// Counter-Strike 1.6 in-game economy (money, weapons shop, buyzones).
pub const ECONOMY: FeatureToken = FeatureToken::from_name("cstrike:economy");

/// Counter-Strike 1.6 SayText color chat formatting (^1 default, ^3 team-color, ^4 green).
pub const COLOR_CHAT: FeatureToken = FeatureToken::from_name("cstrike:color_chat");

/// Counter-Strike 1.6 round timer, objectives, and lifecycle states.
pub const ROUND_TIMER: FeatureToken = FeatureToken::from_name("cstrike:round_timer");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cstrike_feature_tokens() {
        assert_ne!(ECONOMY.raw(), 0);
        assert_ne!(COLOR_CHAT.raw(), 0);
        assert_ne!(ROUND_TIMER.raw(), 0);
        assert_ne!(ECONOMY, COLOR_CHAT);
    }
}
