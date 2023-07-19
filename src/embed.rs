use rocket::{http::ContentType, response::content::RawHtml};
use std::{ffi::OsStr, path::PathBuf, borrow::Cow};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "lib/noVNC"]
struct NoVnc;

#[derive(RustEmbed)]
#[folder = "lib/frontend"]
struct Frontend;

#[get("/")]
pub(crate) fn index() -> Option<RawHtml<Cow<'static, [u8]>>> {
  let asset = Frontend::get("index.html")?;
  Some(RawHtml(asset.data))
}

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

#[get("/<file..>")]
pub(crate) fn frontend_embed(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
  let filename = file.display().to_string();
  let asset = Frontend::get(&filename)?;
  let content_type = file
    .extension()
    .and_then(OsStr::to_str)
    .and_then(ContentType::from_extension)
    .unwrap_or(ContentType::Bytes);

  Some((content_type, asset.data))
}