use crate::applescript::{escape_string, run};
use anyhow::{Context, Result, anyhow, bail};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum ExportFormat {
    /// Export the original file (HEIC, JPG, PNG, etc.) as-is.
    Original,
    /// Export as PNG (uses `sips` to convert from the original).
    Png,
    /// Export as JPG (uses `sips` to convert from the original).
    Jpg,
}

/// Export a photo (by id) into output_dir. If format is Png/Jpg and the original
/// isn't already that format, converts using `sips`. Returns the final output path.
pub fn export_photo(id: &str, output_dir: &Path, format: ExportFormat) -> Result<PathBuf> {
    std::fs::create_dir_all(output_dir)
        .with_context(|| format!("creating output dir {}", output_dir.display()))?;

    let abs_out = output_dir
        .canonicalize()
        .with_context(|| format!("resolving output dir {}", output_dir.display()))?;

    let id_lit = escape_string(id);
    let dir_lit = escape_string(
        abs_out
            .to_str()
            .ok_or_else(|| anyhow!("output dir is not valid UTF-8"))?,
    );

    // Use AppleScript to export the original file to the output dir.
    // Photos will preserve the original filename within the directory.
    let script = format!(
        r#"
with timeout of 600 seconds
    tell application "Photos"
        set p to media item id "{id_lit}"
        set destFolder to (POSIX file "{dir_lit}") as alias
        export {{p}} to destFolder with using originals
        return filename of p
    end tell
end timeout
"#
    );

    let exported_name = run(&script).context("exporting photo")?;
    let exported_path = abs_out.join(&exported_name);

    if !exported_path.exists() {
        bail!(
            "Photos reported export of {exported_name} but file not found at {}",
            exported_path.display()
        );
    }

    match format {
        ExportFormat::Original => Ok(exported_path),
        ExportFormat::Png => convert_with_sips(&exported_path, "png"),
        ExportFormat::Jpg => convert_with_sips(&exported_path, "jpeg"),
    }
}

fn convert_with_sips(input: &Path, sips_format: &str) -> Result<PathBuf> {
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| anyhow!("input file has no stem"))?;
    let ext = match sips_format {
        "jpeg" => "jpg",
        other => other,
    };
    let output = input.with_file_name(format!("{stem}.{ext}"));

    // If already in target format, just return.
    if input
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        == Some(ext.to_string())
    {
        return Ok(input.to_path_buf());
    }

    let status = std::process::Command::new("sips")
        .args([
            "-s",
            "format",
            sips_format,
            input
                .to_str()
                .ok_or_else(|| anyhow!("input path not UTF-8"))?,
            "--out",
            output
                .to_str()
                .ok_or_else(|| anyhow!("output path not UTF-8"))?,
        ])
        .status()
        .context("invoking sips")?;

    if !status.success() {
        bail!(
            "sips conversion to {sips_format} failed (exit {:?})",
            status.code()
        );
    }

    // If original was a different format, remove the original — user asked for PNG/JPG only.
    if input != output {
        let _ = std::fs::remove_file(input);
    }
    Ok(output)
}
