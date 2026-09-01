# goldsrc-game-cstrike

[![CI](https://github.com/goldsrc-rs/goldsrc-game-cstrike/actions/workflows/ci.yml/badge.svg)](https://github.com/goldsrc-rs/goldsrc-game-cstrike/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org/)

Counter-Strike 1.6 game domain crate, weapon constants, equipment, and ReGameDLL extensions for [GoldSrc.rs](https://github.com/goldsrc-rs/goldsrc-rs).

## Features

- **Weapons & Equipment**: Strongly-typed enums for all CS 1.6 weapons (`CsWeapon`), slots (`WeaponSlot`), classnames, and ammo constants.
- **Player Extensions**: Ergonomic `CsPlayerExt` trait providing `has_defuse_kit()`, `give_weapon()`, `cs_team_str()`, and team utilities.
- **Game Rules & Round States**: Enumerations for `RoundState` (FreezePeriod, Active, RoundEnded) and `RoundEndReason` (BombDefused, TargetBombed, VipEscaped, etc.).
- **Zero Raw Panics**: Strict boundary isolation and safe Rust abstractions.

## Usage

Add `goldsrc-game-cstrike` to your `Cargo.toml`:

```toml
[dependencies]
goldsrc-game-cstrike = { git = "https://github.com/goldsrc-rs/goldsrc-game-cstrike.git", branch = "dev" }
```

### Example

```rust
use goldsrc_game_cstrike::{CsPlayerExt, CsWeapon, WeaponSlot};
use goldsrc::prelude::*;

pub fn equip_vip(player: &Player) {
    player.give_weapon(CsWeapon::Deagle);
    player.give_weapon(CsWeapon::M4a1);
    
    if player.has_defuse_kit() {
        println!("Player already has defuse kit");
    }
}
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
