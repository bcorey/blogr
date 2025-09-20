use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Copy a directory recursively
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;

    for entry in WalkDir::new(src) {
        let entry = entry?;
        let path = entry.path();

        if path == src {
            continue;
        }

        let relative_path = path.strip_prefix(src)?;
        let dst_path = dst.join(relative_path);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&dst_path)?;
        } else {
            if let Some(parent) = dst_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &dst_path)?;
        }
    }

    Ok(())
}

/// Optimize CSS by removing comments and extra whitespace
pub fn optimize_css(css: &str) -> String {
    css.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("/*"))
        .collect::<Vec<_>>()
        .join(" ")
        .replace("  ", " ")
        .replace("; ", ";")
        .replace(" {", "{")
        .replace("{ ", "{")
        .replace(" }", "}")
        .replace("} ", "}")
}

/// Optimize JavaScript by removing comments and extra whitespace
pub fn optimize_js(js: &str) -> String {
    js.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//") && !line.starts_with("/*"))
        .collect::<Vec<_>>()
        .join(" ")
        .replace("  ", " ")
}

/// Get MIME type from file extension
pub fn get_mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        Some("eot") => "application/vnd.ms-fontobject",
        Some("pdf") => "application/pdf",
        Some("txt") => "text/plain",
        Some("md") => "text/markdown",
        _ => "application/octet-stream",
    }
}