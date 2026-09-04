# CHANGELOG

All notable changes to `goldsrc-game-cstrike` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.16.0] - 2026-09-04

### Added

- Strongly-typed `CsTeam` enumeration with conversions to/from `goldsrc_api::client::Team` and raw integer IDs.
- Full CS 1.6 weapon enumeration `CsWeapon` covering all CS 1.6 weapons, slots, and canonical classnames.
- `CsPlayerExt` trait providing CS-specific operations on `Player` handles (`cs_team`, `give_weapon`, `has_defuse_kit`, `cs_team_str`).
- `RoundState` and `RoundEndReason` game rule models with TextMsg/SendAudio message token parsing.
- Comprehensive automated test suite for domain models and converters.
- Dual MIT / Apache-2.0 licensing metadata and GitHub Actions CI matrix for Linux and Windows.
