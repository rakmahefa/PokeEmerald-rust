//! Engine-independent gameplay primitives.
//!
//! These types deliberately contain no Bevy components or renderer state. They
//! are shared contracts for map loading, movement, encounters and saves.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Integer tile coordinates are authoritative for gameplay. A Bevy Transform
/// should be treated only as the interpolated visual representation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
    /// Logical elevation/layer from the source map, not world-space metres.
    pub elevation: u8,
}

impl GridPosition {
    pub const fn new(x: i32, y: i32, elevation: u8) -> Self {
        Self { x, y, elevation }
    }

    /// Return the neighbouring cell without changing elevation.
    pub const fn stepped(self, direction: Direction) -> Self {
        let (dx, dy) = direction.delta();
        Self {
            x: self.x + dx,
            y: self.y + dy,
            elevation: self.elevation,
        }
    }
}

/// Cardinal field direction, named in source-map coordinates (Y increases down).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    North,
    #[default]
    South,
    West,
    East,
}

impl Direction {
    pub const fn delta(self) -> (i32, i32) {
        match self {
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::West => (-1, 0),
            Self::East => (1, 0),
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }
}

/// Symbolic map identity from the source catalog. Never derive it from an ECS
/// Entity ID or from a map's position in a loaded vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MapId(String);

impl MapId {
    pub fn new(value: impl Into<String>) -> Result<Self, MapIdError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(MapIdError::Empty);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapIdError {
    Empty,
}

impl fmt::Display for MapIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("map id cannot be empty"),
        }
    }
}

impl std::error::Error for MapIdError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapLocation {
    pub map_id: MapId,
    pub position: GridPosition,
    pub facing: Direction,
}

impl MapLocation {
    pub const fn new(map_id: MapId, position: GridPosition, facing: Direction) -> Self {
        Self {
            map_id,
            position,
            facing,
        }
    }
}

/// Logical locomotion, independent of which key requested it.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MovementMode {
    #[default]
    Walking,
    Running,
    Cycling,
    Surfing,
    Diving,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GamePhase {
    #[default]
    Overworld,
    Dialogue,
    Menu,
    Battle,
    MapTransition,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncounterMethod {
    Walk,
    Surf,
    OldRod,
    GoodRod,
    SuperRod,
    RockSmash,
    Headbutt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncounterSurface {
    Land,
    Water,
}

/// One validated entry from a source wild-encounter table.
///
/// Weight is a relative selection weight, not a percent. The table importer
/// must interpret it according to the pinned source format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncounterSlot {
    pub species_id: u16,
    pub min_level: u8,
    pub max_level: u8,
    pub weight: u16,
}

impl EncounterSlot {
    pub fn validate(&self) -> Result<(), EncounterSlotError> {
        if self.species_id == 0 {
            return Err(EncounterSlotError::MissingSpecies);
        }
        if self.min_level == 0 || self.max_level > 100 || self.min_level > self.max_level {
            return Err(EncounterSlotError::InvalidLevelRange);
        }
        if self.weight == 0 {
            return Err(EncounterSlotError::ZeroWeight);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncounterSlotError {
    MissingSpecies,
    InvalidLevelRange,
    ZeroWeight,
}

impl fmt::Display for EncounterSlotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSpecies => f.write_str("encounter slot has no species"),
            Self::InvalidLevelRange => {
                f.write_str("encounter slot level range must be 1..=100 and min <= max")
            }
            Self::ZeroWeight => f.write_str("encounter slot weight must be non-zero"),
        }
    }
}

impl std::error::Error for EncounterSlotError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_moves_one_cell_in_source_map_coordinates() {
        let start = GridPosition::new(10, 20, 3);
        assert_eq!(
            start.stepped(Direction::North),
            GridPosition::new(10, 19, 3)
        );
        assert_eq!(
            start.stepped(Direction::South),
            GridPosition::new(10, 21, 3)
        );
        assert_eq!(start.stepped(Direction::West), GridPosition::new(9, 20, 3));
        assert_eq!(start.stepped(Direction::East), GridPosition::new(11, 20, 3));
    }

    #[test]
    fn every_direction_has_an_involutive_opposite() {
        for direction in [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ] {
            assert_eq!(direction.opposite().opposite(), direction);
        }
    }

    #[test]
    fn map_id_rejects_empty_identifiers() {
        assert_eq!(MapId::new("  "), Err(MapIdError::Empty));
        assert_eq!(
            MapId::new("LittlerootTown").unwrap().as_str(),
            "LittlerootTown"
        );
    }

    #[test]
    fn encounter_slot_rejects_impossible_ranges_and_zero_weight() {
        let valid = EncounterSlot {
            species_id: 25,
            min_level: 3,
            max_level: 5,
            weight: 20,
        };
        assert!(valid.validate().is_ok());
        assert_eq!(
            EncounterSlot {
                min_level: 6,
                max_level: 5,
                ..valid
            }
            .validate(),
            Err(EncounterSlotError::InvalidLevelRange)
        );
        assert_eq!(
            EncounterSlot {
                species_id: 0,
                ..valid
            }
            .validate(),
            Err(EncounterSlotError::MissingSpecies)
        );
        assert_eq!(
            EncounterSlot { weight: 0, ..valid }.validate(),
            Err(EncounterSlotError::ZeroWeight)
        );
    }

    #[test]
    fn gameplay_contracts_roundtrip_json() {
        let location = MapLocation::new(
            MapId::new("LittlerootTown_MaysHouse_2F").unwrap(),
            GridPosition::new(2, 4, 0),
            Direction::South,
        );
        let encoded = serde_json::to_string(&location).unwrap();
        let decoded: MapLocation = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, location);
    }
}
