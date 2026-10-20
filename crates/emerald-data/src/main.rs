use emerald_data::{MapCatalog, MapLayoutCatalog};
use std::{
    env,
    error::Error,
    io::{Error as IoError, ErrorKind},
    path::PathBuf,
    process,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("map catalog import failed: {error}");
        process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut source = None::<PathBuf>;
    let mut output = None::<PathBuf>;
    let mut pret_commit = None::<String>;
    let mut layout_output_dir = None::<PathBuf>;
    let mut args = env::args().skip(1);

    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--source" => {
                source = Some(PathBuf::from(args.next().ok_or_else(|| {
                    IoError::new(ErrorKind::InvalidInput, "--source requires a directory")
                })?));
            }
            "--output" => {
                output = Some(PathBuf::from(args.next().ok_or_else(|| {
                    IoError::new(ErrorKind::InvalidInput, "--output requires a file path")
                })?));
            }
            "--layout-output-dir" => {
                layout_output_dir = Some(PathBuf::from(args.next().ok_or_else(|| {
                    IoError::new(
                        ErrorKind::InvalidInput,
                        "--layout-output-dir requires a directory",
                    )
                })?));
            }
            "--pret-commit" => {
                pret_commit = Some(args.next().ok_or_else(|| {
                    IoError::new(
                        ErrorKind::InvalidInput,
                        "--pret-commit requires a commit hash",
                    )
                })?);
            }
            "--help" | "-h" => {
                println!(
                    "Usage: emerald-data --source PRET_CHECKOUT --output OUTPUT_JSON --pret-commit SHA [--layout-output-dir DIR]"
                );
                return Ok(());
            }
            unknown => {
                return Err(IoError::new(
                    ErrorKind::InvalidInput,
                    format!("unknown argument: {unknown}"),
                )
                .into());
            }
        }
    }

    let source = source.ok_or_else(|| {
        IoError::new(
            ErrorKind::InvalidInput,
            "missing required --source directory",
        )
    })?;
    let output = output
        .ok_or_else(|| IoError::new(ErrorKind::InvalidInput, "missing required --output path"))?;
    let pret_commit = pret_commit.ok_or_else(|| {
        IoError::new(
            ErrorKind::InvalidInput,
            "missing required --pret-commit hash",
        )
    })?;

    let catalog = MapCatalog::import_from_pret(&source, &pret_commit)?;
    catalog.write_json(&output)?;
    if let Some(output_dir) = layout_output_dir {
        let (layouts, payload) = MapLayoutCatalog::import_from_pret(&source, &pret_commit)?;
        let layout_catalog_path = output_dir.join("map_layouts.json");
        let layout_payload_path = output_dir.join("map_layouts.bin");
        layouts.write_files(&layout_catalog_path, &layout_payload_path, &payload)?;
        println!(
            "Imported {} layouts ({} payload bytes) into {}",
            layouts.layout_count,
            layouts.payload_size,
            output_dir.display()
        );
    }
    println!(
        "Imported {} map records across {} groups from pret commit {} into {}",
        catalog.map_count,
        catalog.groups.len(),
        catalog.source.commit,
        output.display()
    );
    Ok(())
}
