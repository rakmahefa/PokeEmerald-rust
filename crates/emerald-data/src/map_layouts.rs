//! Validated importer/decoder for the packed map layouts used by pret/pokeemerald.
//!
//! The source layout stores one little-endian u16 per cell. Bits 0..9 are the
//! metatile ID, bits 10..11 are collision flags, and bits 12..15 are elevation.
//! The payload remains separate from JSON metadata to keep the bundle compact.

use super::{is_full_commit_hash, read_bounded_json, validate_path_component, DataError, SourceRevision};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    error::Error,
    fmt,
    fs,
    path::{Path, PathBuf},
};

pub const MAP_LAYOUT_SCHEMA_VERSION: u32 = 1;
const MAX_LAYOUT_INDEX_BYTES: u64 = 4 * 1024 * 1024;
const MAX_LAYOUT_FILE_BYTES: u64 = 8 * 1024 * 1024;
const MAX_LAYOUT_PAYLOAD_BYTES: u64 = 128 * 1024 * 1024;
const MAX_LAYOUT_COUNT: usize = 2_048;
const MAX_LAYOUT_DIMENSION: u32 = 4_096;
const MAX_LAYOUT_CELLS: u64 = 1_048_576;
const BORDER_BYTES: u64 = 8;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapLayoutCatalog {
    pub schema_version: u32,
    pub source: SourceRevision,
    pub layout_count: usize,
    /// Length of the companion map_layouts.bin payload, in bytes.
    pub payload_size: u64,
    pub layouts: Vec<MapLayoutDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapLayoutDescriptor {
    pub id: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub primary_tileset: String,
    pub secondary_tileset: String,
    pub map_offset: u64,
    pub map_length: u64,
    pub border_offset: u64,
    pub border_length: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapCell {
    /// Original packed 16-bit value.
    pub raw: u16,
    pub metatile_id: u16,
    pub collision: u8,
    pub elevation: u8,
}

impl MapCell {
    pub const fn from_raw(raw: u16) -> Self {
        Self {
            raw,
            metatile_id: raw & 0x03ff,
            collision: ((raw & 0x0c00) >> 10) as u8,
            elevation: ((raw & 0xf000) >> 12) as u8,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedMapLayout {
    pub descriptor: MapLayoutDescriptor,
    pub cells: Vec<MapCell>,
    pub border: Vec<MapCell>,
}

impl DecodedMapLayout {
    pub fn cell(&self, x: u32, y: u32) -> Option<MapCell> {
        if x >= self.descriptor.width || y >= self.descriptor.height {
            return None;
        }
        let index = (y as usize)
            .checked_mul(self.descriptor.width as usize)?
            .checked_add(x as usize)?;
        self.cells.get(index).copied()
    }
}

#[derive(Debug, Deserialize)]
struct RawLayoutIndex {
    layouts: Vec<RawLayoutDescriptor>,
}

#[derive(Debug, Deserialize)]
struct RawLayoutDescriptor {
    id: String,
    name: String,
    width: u32,
    height: u32,
    primary_tileset: String,
    secondary_tileset: String,
    border_filepath: String,
    blockdata_filepath: String,
}

impl MapLayoutCatalog {
    /// Import layouts.json plus each referenced map.bin and border.bin.
    ///
    /// The result is a JSON descriptor catalog and one compact binary payload.
    /// The caller decides where these files are written; game source assets are
    /// never copied into the public source repository by this importer.
    pub fn import_from_pret(
        source_root: impl AsRef<Path>,
        pret_commit: &str,
    ) -> Result<(Self, Vec<u8>), DataError> {
        if !is_full_commit_hash(pret_commit) {
            return Err(DataError::new(
                "pret commit must be a 40-character hexadecimal commit hash",
            ));
        }

        let source_root = source_root.as_ref();
        let index_path = source_root.join("data").join("layouts").join("layouts.json");
        let index_bytes = read_bounded_json(&index_path)?;
        if index_bytes.len() as u64 > MAX_LAYOUT_INDEX_BYTES {
            return Err(DataError::new(format!(
                "layout index is too large: {} bytes",
                index_bytes.len()
            )));
        }
        let raw_index: RawLayoutIndex =
            serde_json::from_slice(&index_bytes).map_err(|error| {
                DataError::new(format!(
                    "could not parse layout index {}: {error}",
                    index_path.display()
                ))
            })?;
        if raw_index.layouts.is_empty() || raw_index.layouts.len() > MAX_LAYOUT_COUNT {
            return Err(DataError::new(format!(
                "layout index count must be between 1 and {MAX_LAYOUT_COUNT}"
            )));
        }

        let mut ids = HashSet::new();
        let mut names = HashSet::new();
        let mut payload = Vec::new();
        let mut layouts = Vec::with_capacity(raw_index.layouts.len());

        for raw in raw_index.layouts {
            validate_path_component(&raw.id, "layout id")?;
            validate_path_component(&raw.name, "layout name")?;
            validate_path_component(&raw.primary_tileset, "primary tileset")?;
            validate_path_component(&raw.secondary_tileset, "secondary tileset")?;
            if !ids.insert(raw.id.clone()) || !names.insert(raw.name.clone()) {
                return Err(DataError::new(format!(
                    "layout index contains a duplicate id or name: {:?}",
                    raw.id
                )));
            }
            if raw.width == 0
                || raw.height == 0
                || raw.width > MAX_LAYOUT_DIMENSION
                || raw.height > MAX_LAYOUT_DIMENSION
            {
                return Err(DataError::new(format!(
                    "layout {} has invalid dimensions {}x{}",
                    raw.id, raw.width, raw.height
                )));
            }

            let cell_count = u64::from(raw.width)
                .checked_mul(u64::from(raw.height))
                .ok_or_else(|| DataError::new(format!("layout {} dimensions overflow", raw.id)))?;
            if cell_count > MAX_LAYOUT_CELLS {
                return Err(DataError::new(format!(
                    "layout {} is too large: {} cells",
                    raw.id, cell_count
                )));
            }
            let expected_map_length = cell_count
                .checked_mul(2)
                .ok_or_else(|| DataError::new(format!("layout {} byte size overflows", raw.id)))?;

            let map_path = resolve_layout_asset_path(source_root, &raw.blockdata_filepath, "map.bin")?;
            let border_path =
                resolve_layout_asset_path(source_root, &raw.border_filepath, "border.bin")?;
            let map_bytes = read_bounded_binary(&map_path)?;
            let border_bytes = read_bounded_binary(&border_path)?;

            if map_bytes.len() as u64 != expected_map_length {
                return Err(DataError::new(format!(
                    "layout {} expects {} bytes from {}x{} cells, found {} bytes in {}",
                    raw.id,
                    expected_map_length,
                    raw.width,
                    raw.height,
                    map_bytes.len(),
                    map_path.display()
                )));
            }
            if border_bytes.len() as u64 != BORDER_BYTES {
                return Err(DataError::new(format!(
                    "layout {} expects a {}-byte 2x2 border, found {} bytes in {}",
                    raw.id,
                    BORDER_BYTES,
                    border_bytes.len(),
                    border_path.display()
                )));
            }

            if payload.len() % 2 != 0 {
                return Err(DataError::new("internal layout payload lost u16 alignment"));
            }
            let map_offset = payload.len() as u64;
            payload.extend_from_slice(&map_bytes);
            let border_offset = payload.len() as u64;
            payload.extend_from_slice(&border_bytes);
            if payload.len() as u64 > MAX_LAYOUT_PAYLOAD_BYTES {
                return Err(DataError::new(format!(
                    "layout payload exceeds {} bytes",
                    MAX_LAYOUT_PAYLOAD_BYTES
                )));
            }

            layouts.push(MapLayoutDescriptor {
                id: raw.id,
                name: raw.name,
                width: raw.width,
                height: raw.height,
                primary_tileset: raw.primary_tileset,
                secondary_tileset: raw.secondary_tileset,
                map_offset,
                map_length: expected_map_length,
                border_offset,
                border_length: BORDER_BYTES,
            });
        }

        let catalog = Self {
            schema_version: MAP_LAYOUT_SCHEMA_VERSION,
            source: SourceRevision {
                project: "pret/pokeemerald".to_owned(),
                commit: pret_commit.to_ascii_lowercase(),
            },
            layout_count: layouts.len(),
            payload_size: payload.len() as u64,
            layouts,
        };
        catalog.validate()?;
        Ok((catalog, payload))
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, DataError> {
        let path = path.as_ref();
        let bytes = read_bounded_json(path)?;
        let catalog: Self = serde_json::from_slice(&bytes).map_err(|error| {
            DataError::new(format!(
                "could not parse map layout catalog {}: {error}",
                path.display()
            ))
        })?;
        catalog.validate()?;
        Ok(catalog)
    }

    pub fn validate(&self) -> Result<(), DataError> {
        if self.schema_version != MAP_LAYOUT_SCHEMA_VERSION {
            return Err(DataError::new(format!(
                "unsupported layout catalog schema {}; expected {}",
                self.schema_version, MAP_LAYOUT_SCHEMA_VERSION
            )));
        }
        if self.source.project != "pret/pokeemerald" || !is_full_commit_hash(&self.source.commit) {
            return Err(DataError::new("layout catalog source provenance is invalid"));
        }
        if self.layouts.is_empty()
            || self.layout_count != self.layouts.len()
            || self.layouts.len() > MAX_LAYOUT_COUNT
        {
            return Err(DataError::new(format!(
                "layout catalog count mismatch: declared {}, contains {}",
                self.layout_count,
                self.layouts.len()
            )));
        }
        if self.payload_size > MAX_LAYOUT_PAYLOAD_BYTES {
            return Err(DataError::new("layout payload size exceeds the configured limit"));
        }

        let mut ids = HashSet::new();
        let mut names = HashSet::new();
        let mut ranges = Vec::<(u64, u64)>::with_capacity(self.layouts.len() * 2);
        for layout in &self.layouts {
            validate_path_component(&layout.id, "layout id")?;
            validate_path_component(&layout.name, "layout name")?;
            validate_path_component(&layout.primary_tileset, "primary tileset")?;
            validate_path_component(&layout.secondary_tileset, "secondary tileset")?;
            if !ids.insert(layout.id.as_str()) || !names.insert(layout.name.as_str()) {
                return Err(DataError::new(format!(
                    "layout catalog contains duplicate id or name: {:?}",
                    layout.id
                )));
            }
            if layout.width == 0
                || layout.height == 0
                || layout.width > MAX_LAYOUT_DIMENSION
                || layout.height > MAX_LAYOUT_DIMENSION
            {
                return Err(DataError::new(format!(
                    "layout {} has invalid dimensions",
                    layout.id
                )));
            }
            let expected_map_length = u64::from(layout.width)
                .checked_mul(u64::from(layout.height))
                .and_then(|cells| cells.checked_mul(2))
                .filter(|_| {
                    u64::from(layout.width) * u64::from(layout.height) <= MAX_LAYOUT_CELLS
                })
                .ok_or_else(|| DataError::new(format!("layout {} dimensions are invalid", layout.id)))?;
            if layout.map_length != expected_map_length || layout.border_length != BORDER_BYTES {
                return Err(DataError::new(format!(
                    "layout {} has inconsistent map/border byte lengths",
                    layout.id
                )));
            }
            add_payload_range(&mut ranges, layout.map_offset, layout.map_length, self.payload_size)?;
            add_payload_range(&mut ranges, layout.border_offset, layout.border_length, self.payload_size)?;
        }

        ranges.sort_unstable_by_key(|range| range.0);
        let mut expected_offset = 0_u64;
        for (offset, length) in ranges {
            if offset != expected_offset {
                return Err(DataError::new(
                    "layout payload offsets overlap or leave an unreferenced gap",
                ));
            }
            expected_offset = expected_offset
                .checked_add(length)
                .ok_or_else(|| DataError::new("layout payload offset overflow"))?;
        }
        if expected_offset != self.payload_size {
            return Err(DataError::new(format!(
                "layout payload ranges total {} bytes, catalog declares {}",
                expected_offset, self.payload_size
            )));
        }
        Ok(())
    }

    pub fn find(&self, id: &str) -> Option<&MapLayoutDescriptor> {
        self.layouts.iter().find(|layout| layout.id == id)
    }

    /// Decode one layout and its 2x2 border after validating the packed payload.
    pub fn decode_layout(
        &self,
        id: &str,
        payload: &[u8],
    ) -> Result<DecodedMapLayout, DataError> {
        self.validate()?;
        if payload.len() as u64 != self.payload_size {
            return Err(DataError::new(format!(
                "layout payload size mismatch: catalog says {}, file contains {}",
                self.payload_size,
                payload.len()
            )));
        }
        let descriptor = self
            .find(id)
            .cloned()
            .ok_or_else(|| DataError::new(format!("layout id not found: {id}")))?;
        let map_bytes = payload_slice(payload, descriptor.map_offset, descriptor.map_length)?;
        let border_bytes = payload_slice(payload, descriptor.border_offset, descriptor.border_length)?;
        let cells = map_bytes
            .chunks_exact(2)
            .map(|bytes| MapCell::from_raw(u16::from_le_bytes([bytes[0], bytes[1]])))
            .collect();
        let border = border_bytes
            .chunks_exact(2)
            .map(|bytes| MapCell::from_raw(u16::from_le_bytes([bytes[0], bytes[1]])))
            .collect();
        Ok(DecodedMapLayout {
            descriptor,
            cells,
            border,
        })
    }

    /// Write the binary payload first and the JSON catalog last.
    ///
    /// The JSON file acts as the bundle's commit marker: if writing the payload
    /// fails, a new catalog is not published alongside a partial payload.
    pub fn write_files(
        &self,
        catalog_path: impl AsRef<Path>,
        payload_path: impl AsRef<Path>,
        payload: &[u8],
    ) -> Result<(), DataError> {
        self.validate()?;
        if payload.len() as u64 != self.payload_size {
            return Err(DataError::new(format!(
                "refusing to write layout payload of {} bytes; catalog requires {}",
                payload.len(),
                self.payload_size
            )));
        }
        let payload_path = payload_path.as_ref();
        if let Some(parent) = payload_path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent).map_err(|error| {
                DataError::new(format!(
                    "could not create layout payload directory {}: {error}",
                    parent.display()
                ))
            })?;
        }
        fs::write(payload_path, payload).map_err(|error| {
            DataError::new(format!(
                "could not write layout payload {}: {error}",
                payload_path.display()
            ))
        })?;

        let catalog_path = catalog_path.as_ref();
        if let Some(parent) = catalog_path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent).map_err(|error| {
                DataError::new(format!(
                    "could not create layout catalog directory {}: {error}",
                    parent.display()
                ))
            })?;
        }
        let mut json = serde_json::to_vec_pretty(self)
            .map_err(|error| DataError::new(format!("could not serialize layout catalog: {error}")))?;
        json.push(b'\n');
        fs::write(catalog_path, json).map_err(|error| {
            DataError::new(format!(
                "could not write layout catalog {}: {error}",
                catalog_path.display()
            ))
        })
    }
}

fn add_payload_range(
    ranges: &mut Vec<(u64, u64)>,
    offset: u64,
    length: u64,
    payload_size: u64,
) -> Result<(), DataError> {
    if length == 0 || offset % 2 != 0 {
        return Err(DataError::new("layout payload range is empty or misaligned"));
    }
    let end = offset
        .checked_add(length)
        .ok_or_else(|| DataError::new("layout payload range overflow"))?;
    if end > payload_size {
        return Err(DataError::new("layout payload range extends past end of file"));
    }
    ranges.push((offset, length));
    Ok(())
}

fn payload_slice(payload: &[u8], offset: u64, length: u64) -> Result<&[u8], DataError> {
    let start = usize::try_from(offset)
        .map_err(|_| DataError::new("layout payload offset does not fit this platform"))?;
    let length = usize::try_from(length)
        .map_err(|_| DataError::new("layout payload length does not fit this platform"))?;
    let end = start
        .checked_add(length)
        .ok_or_else(|| DataError::new("layout payload slice overflow"))?;
    payload
        .get(start..end)
        .ok_or_else(|| DataError::new("layout payload slice extends past end of file"))
}

fn read_bounded_binary(path: &Path) -> Result<Vec<u8>, DataError> {
    let metadata = fs::metadata(path)
        .map_err(|error| DataError::new(format!("could not read {}: {error}", path.display())))?;
    if metadata.len() > MAX_LAYOUT_FILE_BYTES {
        return Err(DataError::new(format!(
            "refusing oversized layout binary {} ({} bytes)",
            path.display(),
            metadata.len()
        )));
    }
    fs::read(path).map_err(|error| {
        DataError::new(format!("could not read layout binary {}: {error}", path.display()))
    })
}

fn resolve_layout_asset_path(
    source_root: &Path,
    relative_path: &str,
    expected_filename: &str,
) -> Result<PathBuf, DataError> {
    let parts = relative_path.split('/').collect::<Vec<_>>();
    if parts.len() != 4
        || parts[0] != "data"
        || parts[1] != "layouts"
        || parts[3] != expected_filename
    {
        return Err(DataError::new(format!(
            "layout asset path must be data/layouts/<name>/{expected_filename}: {relative_path:?}"
        )));
    }
    validate_path_component(parts[2], "layout directory")?;
    Ok(source_root.join("data").join("layouts").join(parts[2]).join(expected_filename))
}

impl fmt::Display for MapCell {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "metatile={:#05x}, collision={}, elevation={}",
            self.metatile_id, self.collision, self.elevation
        )
    }
}

impl Error for DecodedMapLayout {}


#[cfg(test)]
mod map_layout_tests {
    use super::*;
    use std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

    fn fixture_root() -> PathBuf {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).expect("clock after epoch").as_nanos();
        let root = std::env::temp_dir().join(format!("emerald-layout-fixture-{}-{nonce}", std::process::id()));
        let layout_dir = root.join("data/layouts/TestRoom");
        fs::create_dir_all(&layout_dir).expect("create test layout");
        fs::write(
            root.join("data/layouts/layouts.json"),
            r#"{
              "layouts": [
                {
                  "id": "LAYOUT_TEST_ROOM",
                  "name": "TestRoom_Layout",
                  "width": 2,
                  "height": 2,
                  "primary_tileset": "gTileset_General",
                  "secondary_tileset": "gTileset_Building",
                  "border_filepath": "data/layouts/TestRoom/border.bin",
                  "blockdata_filepath": "data/layouts/TestRoom/map.bin"
                }
              ]
            }"#,
        ).expect("write layout index");
        let words = [0x7955_u16, 0x040a, 0x03ff, 0xb123];
        let map_bytes = words.into_iter().flat_map(u16::to_le_bytes).collect::<Vec<_>>();
        fs::write(layout_dir.join("map.bin"), map_bytes).expect("write map data");
        let border_words = [1_u16, 2, 3, 4];
        let border_bytes = border_words.into_iter().flat_map(u16::to_le_bytes).collect::<Vec<_>>();
        fs::write(layout_dir.join("border.bin"), border_bytes).expect("write border data");
        root
    }

    #[test]
    fn importer_preserves_packed_map_cells_and_border() {
        let root = fixture_root();
        let (catalog, payload) = MapLayoutCatalog::import_from_pret(&root, &"a".repeat(40))
            .expect("import fixture layouts");
        assert_eq!(catalog.layout_count, 1);
        assert_eq!(catalog.payload_size, 24);
        let decoded = catalog.decode_layout("LAYOUT_TEST_ROOM", &payload).expect("decode layout");
        assert_eq!(decoded.descriptor.width, 2);
        assert_eq!(decoded.descriptor.height, 2);
        assert_eq!(decoded.cells.len(), 4);
        assert_eq!(decoded.cells[0], MapCell { raw: 0x7955, metatile_id: 0x155, collision: 2, elevation: 7 });
        assert_eq!(decoded.cells[1], MapCell { raw: 0x040a, metatile_id: 10, collision: 1, elevation: 0 });
        assert_eq!(decoded.cells[2].metatile_id, 0x03ff);
        assert_eq!(decoded.cells[3].elevation, 11);
        assert_eq!(decoded.border.iter().map(|cell| cell.raw).collect::<Vec<_>>(), vec![1, 2, 3, 4]);
        assert_eq!(decoded.cell(1, 0), Some(decoded.cells[1]));
        assert_eq!(decoded.cell(2, 0), None);
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn catalog_and_binary_payload_round_trip() {
        let root = fixture_root();
        let (catalog, payload) = MapLayoutCatalog::import_from_pret(&root, &"b".repeat(40))
            .expect("import fixture layouts");
        let catalog_path = root.join("output/map_layouts.json");
        let payload_path = root.join("output/map_layouts.bin");
        catalog.write_files(&catalog_path, &payload_path, &payload).expect("write bundle");
        let loaded = MapLayoutCatalog::load(&catalog_path).expect("load catalog");
        let reloaded_payload = fs::read(&payload_path).expect("read payload");
        assert_eq!(loaded, catalog);
        assert_eq!(
            loaded.decode_layout("LAYOUT_TEST_ROOM", &reloaded_payload).expect("decode loaded bundle"),
            catalog.decode_layout("LAYOUT_TEST_ROOM", &payload).expect("decode original bundle")
        );
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn importer_rejects_wrong_map_length_and_path_traversal() {
        let root = fixture_root();
        fs::write(root.join("data/layouts/TestRoom/map.bin"), [0_u8; 2]).expect("truncate map");
        assert!(MapLayoutCatalog::import_from_pret(&root, &"c".repeat(40)).is_err());
        fs::remove_dir_all(&root).expect("remove fixture");

        let root = fixture_root();
        let index = root.join("data/layouts/layouts.json");
        let source = fs::read_to_string(&index).expect("read layout index");
        let updated = source.replace(
            "data/layouts/TestRoom/map.bin",
            "data/layouts/../../outside/map.bin",
        );
        fs::write(&index, updated).expect("write unsafe path");
        assert!(MapLayoutCatalog::import_from_pret(&root, &"d".repeat(40)).is_err());
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn catalog_validation_rejects_overlapping_or_inconsistent_ranges() {
        let root = fixture_root();
        let (mut catalog, _) = MapLayoutCatalog::import_from_pret(&root, &"e".repeat(40))
            .expect("import fixture layouts");
        catalog.layouts[0].border_offset = catalog.layouts[0].map_offset;
        assert!(catalog.validate().is_err());
        fs::remove_dir_all(root).expect("remove fixture");
    }
}
