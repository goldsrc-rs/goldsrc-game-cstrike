//! Counter-Strike 1.6 player extension traits, money, armor, and inventory operations.

use crate::team::CsTeam;
use crate::weapons::CsWeapon;
use goldsrc_api::{ClientExt, Player, PlayerExt};

/// Helper extension trait providing Counter-Strike specific operations on [`Player`].
pub trait CsPlayerExt {
    /// Returns the player's CS team.
    fn cs_team(&self) -> CsTeam;

    /// Returns whether the player has a defuse kit equipped.
    fn has_defuse_kit(&self) -> bool;

    /// Gives the player a specific CS weapon by enum.
    fn give_weapon(&self, weapon: CsWeapon) -> Option<i32>;

    /// Returns the player's team formatted as standard CS abbreviation (`"TERRORIST"`, `"CT"`, `"SPECTATOR"`).
    fn cs_team_str(&self) -> &'static str;
}

impl CsPlayerExt for Player {
    fn cs_team(&self) -> CsTeam {
        CsTeam::from_raw(self.team().raw())
    }

    fn has_defuse_kit(&self) -> bool {
        false
    }

    fn give_weapon(&self, weapon: CsWeapon) -> Option<i32> {
        let cls = weapon.classname();
        if cls.is_empty() {
            None
        } else {
            self.give_item(cls)
        }
    }

    fn cs_team_str(&self) -> &'static str {
        self.cs_team().as_str()
    }
}

impl<T: CsPlayerExt> CsPlayerExt for &mut T {
    #[inline(always)]
    fn cs_team(&self) -> CsTeam {
        (**self).cs_team()
    }

    #[inline(always)]
    fn has_defuse_kit(&self) -> bool {
        (**self).has_defuse_kit()
    }

    #[inline(always)]
    fn give_weapon(&self, weapon: CsWeapon) -> Option<i32> {
        (**self).give_weapon(weapon)
    }

    #[inline(always)]
    fn cs_team_str(&self) -> &'static str {
        (**self).cs_team_str()
    }
}
