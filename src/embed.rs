use rocket::http::ContentType;
use rust_embed::RustEmbed;
use std::{borrow::Cow, ffi::OsStr, path::PathBuf};

#[derive(RustEmbed)]
#[folder = "lib/noVNC"]
struct NoVnc;

#[get("/noVNC/<file..>")]
pub(crate) fn novnc_embed(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
    let filename = file.display().to_string();
    let asset = NoVnc::get(&filename)?;
    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);

    Some((content_type, asset.data))
}
