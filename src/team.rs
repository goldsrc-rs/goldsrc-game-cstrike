//! Counter-Strike 1.6 team identifiers and conversions.

use goldsrc_api::client::Team;

/// Counter-Strike 1.6 teams.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum CsTeam {
    /// Player has not chosen a team yet.
    Unassigned = 0,
    /// Terrorist team (ID = 1).
    Terrorist = 1,
    /// Counter-Terrorist team (ID = 2).
    Ct = 2,
    /// Spectator mode (ID = 3).
    Spectator = 3,
}

impl CsTeam {
    /// Short display name for the team (e.g. `"TERRORIST"`, `"CT"`).
    pub const fn as_str(&self) -> &'static str {
        match self {
            CsTeam::Terrorist => "TERRORIST",
            CsTeam::Ct => "CT",
            CsTeam::Spectator => "SPECTATOR",
            CsTeam::Unassigned => "UNASSIGNED",
        }
    }

    /// Converts raw integer ID to [`CsTeam`].
    pub const fn from_raw(id: i32) -> Self {
        match id {
            1 => CsTeam::Terrorist,
            2 => CsTeam::Ct,
            3 => CsTeam::Spectator,
            _ => CsTeam::Unassigned,
        }
    }

    /// Returns the raw integer value of the team.
    pub const fn raw(&self) -> i32 {
        *self as i32
    }

    /// Returns `true` if the team is playing (either Terrorist or CT).
    pub const fn is_playing(&self) -> bool {
        matches!(self, CsTeam::Terrorist | CsTeam::Ct)
    }

    /// Returns the opposing playing team, or `None` if unassigned/spectator.
    pub const fn opponent(&self) -> Option<CsTeam> {
        match self {
            CsTeam::Terrorist => Some(CsTeam::Ct),
            CsTeam::Ct => Some(CsTeam::Terrorist),
            _ => None,
        }
    }
}

impl std::fmt::Display for CsTeam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<Team> for CsTeam {
    fn from(team: Team) -> Self {
        CsTeam::from_raw(team.raw())
    }
}

impl From<CsTeam> for Team {
    fn from(team: CsTeam) -> Self {
        Team::new(team.raw())
    }
}

impl From<i32> for CsTeam {
    fn from(id: i32) -> Self {
        CsTeam::from_raw(id)
    }
}

impl From<CsTeam> for i32 {
    fn from(team: CsTeam) -> Self {
        team.raw()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csteam_conversions() {
        assert_eq!(CsTeam::from_raw(1), CsTeam::Terrorist);
        assert_eq!(CsTeam::from_raw(2), CsTeam::Ct);
        assert_eq!(CsTeam::from_raw(3), CsTeam::Spectator);
        assert_eq!(CsTeam::from_raw(0), CsTeam::Unassigned);
        assert_eq!(CsTeam::from_raw(99), CsTeam::Unassigned);

        let team_api: Team = CsTeam::Ct.into();
        assert_eq!(team_api.raw(), 2);
        let cs_team: CsTeam = team_api.into();
        assert_eq!(cs_team, CsTeam::Ct);

        assert!(CsTeam::Terrorist.is_playing());
        assert!(CsTeam::Ct.is_playing());
        assert!(!CsTeam::Spectator.is_playing());
        assert_eq!(CsTeam::Terrorist.opponent(), Some(CsTeam::Ct));
        assert_eq!(CsTeam::Ct.opponent(), Some(CsTeam::Terrorist));
        assert_eq!(CsTeam::Spectator.opponent(), None);
    }
}
