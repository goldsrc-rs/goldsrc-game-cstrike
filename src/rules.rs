//! Counter-Strike 1.6 game rules, round phases, win conditions, and event parsers.

use crate::team::CsTeam;

/// Current CS 1.6 round state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RoundState {
    /// Freeze period before round start (players cannot move).
    #[default]
    FreezePeriod,
    /// Active gameplay in progress.
    Active,
    /// Round concluded (waiting for reset/respawn).
    RoundEnded,
}

/// Round termination reason in Counter-Strike 1.6.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoundEndReason {
    /// Target has been bombed by terrorists (`#CSTRIKE_Target_Bombed`).
    TargetBombed,
    /// VIP escaped successfully (`#CSTRIKE_VIP_Escaped`).
    VipEscaped,
    /// VIP was assassinated (`#CSTRIKE_VIP_Assassinated`).
    VipAssassinated,
    /// Terrorists escaped.
    TerroristsEscaped,
    /// CTs prevented terrorists from escaping.
    CtPreventEscape,
    /// Escaping terrorists were stopped.
    TerroristsStopped,
    /// Bomb defused by CT (`#CSTRIKE_Bomb_Defused`).
    BombDefused,
    /// All terrorists eliminated (`#CSTRIKE_CT_Win`).
    CtWin,
    /// All counter-terrorists eliminated (`#CSTRIKE_Terrorist_Win`).
    TerroristWin,
    /// Round drawn / time expired with no objective met (`#CSTRIKE_Round_Draw`).
    RoundDraw,
    /// All hostages rescued (`#CSTRIKE_All_Hostages_Rescued`).
    AllHostagesRescued,
    /// Target saved (time ran out before bomb planted) (`#CSTRIKE_Target_Saved`).
    TargetSaved,
    /// Hostages were not rescued in time (`#CSTRIKE_Hostages_Not_Rescued`).
    HostagesNotRescued,
    /// Terrorists did not escape in time (`#CSTRIKE_Terrorists_Not_Escaped`).
    TerroristsNotEscaped,
}

impl RoundEndReason {
    /// Returns the team that won the round based on the termination reason.
    pub const fn winning_team(&self) -> Option<CsTeam> {
        match self {
            RoundEndReason::TargetBombed
            | RoundEndReason::VipAssassinated
            | RoundEndReason::TerroristsEscaped
            | RoundEndReason::TerroristWin => Some(CsTeam::Terrorist),

            RoundEndReason::VipEscaped
            | RoundEndReason::CtPreventEscape
            | RoundEndReason::TerroristsStopped
            | RoundEndReason::BombDefused
            | RoundEndReason::CtWin
            | RoundEndReason::AllHostagesRescued
            | RoundEndReason::TargetSaved
            | RoundEndReason::HostagesNotRescued
            | RoundEndReason::TerroristsNotEscaped => Some(CsTeam::Ct),

            RoundEndReason::RoundDraw => None,
        }
    }

    /// Parses a round end message token from GoldSrc TextMsg / SendAudio.
    pub fn from_message_token(token: &str) -> Option<Self> {
        match token {
            "#Target_Bombed" | "#CSTRIKE_Target_Bombed" => Some(RoundEndReason::TargetBombed),
            "#VIP_Escaped" | "#CSTRIKE_VIP_Escaped" => Some(RoundEndReason::VipEscaped),
            "#VIP_Assassinated" | "#CSTRIKE_VIP_Assassinated" => {
                Some(RoundEndReason::VipAssassinated)
            }
            "#Terrorists_Escaped" | "#CSTRIKE_Terrorists_Escaped" => {
                Some(RoundEndReason::TerroristsEscaped)
            }
            "#CTs_PreventEscape" | "#CSTRIKE_CT_PreventEscape" => {
                Some(RoundEndReason::CtPreventEscape)
            }
            "#Escaping_Terrorists_Neutralized" | "#CSTRIKE_Terrorists_Stopped" => {
                Some(RoundEndReason::TerroristsStopped)
            }
            "#Bomb_Defused" | "#CSTRIKE_Bomb_Defused" => Some(RoundEndReason::BombDefused),
            "#CTs_Win" | "#CSTRIKE_CT_Win" => Some(RoundEndReason::CtWin),
            "#Terrorists_Win" | "#CSTRIKE_Terrorist_Win" => Some(RoundEndReason::TerroristWin),
            "#Round_Draw" | "#CSTRIKE_Round_Draw" => Some(RoundEndReason::RoundDraw),
            "#All_Hostages_Rescued" | "#CSTRIKE_All_Hostages_Rescued" => {
                Some(RoundEndReason::AllHostagesRescued)
            }
            "#Target_Saved" | "#CSTRIKE_Target_Saved" => Some(RoundEndReason::TargetSaved),
            "#Hostages_Not_Rescued" | "#CSTRIKE_Hostages_Not_Rescued" => {
                Some(RoundEndReason::HostagesNotRescued)
            }
            "#Terrorists_Not_Escaped" | "#CSTRIKE_Terrorists_Not_Escaped" => {
                Some(RoundEndReason::TerroristsNotEscaped)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_end_reason_winners() {
        assert_eq!(
            RoundEndReason::TargetBombed.winning_team(),
            Some(CsTeam::Terrorist)
        );
        assert_eq!(RoundEndReason::BombDefused.winning_team(), Some(CsTeam::Ct));
        assert_eq!(RoundEndReason::CtWin.winning_team(), Some(CsTeam::Ct));
        assert_eq!(
            RoundEndReason::TerroristWin.winning_team(),
            Some(CsTeam::Terrorist)
        );
        assert_eq!(RoundEndReason::RoundDraw.winning_team(), None);
    }

    #[test]
    fn test_round_end_message_parsing() {
        assert_eq!(
            RoundEndReason::from_message_token("#CSTRIKE_Target_Bombed"),
            Some(RoundEndReason::TargetBombed)
        );
        assert_eq!(
            RoundEndReason::from_message_token("#Bomb_Defused"),
            Some(RoundEndReason::BombDefused)
        );
        assert_eq!(
            RoundEndReason::from_message_token("#CTs_Win"),
            Some(RoundEndReason::CtWin)
        );
        assert_eq!(RoundEndReason::from_message_token("unknown_message"), None);
    }
}
