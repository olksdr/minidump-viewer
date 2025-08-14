use serde::Serialize;
use symbolic::debuginfo::Object;

#[derive(Serialize)]
pub struct Meta {
    pub kind: String,
    pub arch: Option<String>,
    pub debug_id: Option<String>,
    pub code_id: Option<String>,
}

// Optional: prove `symbolic` compiles on Wasm and let users drop a PDB/ELF/Mach-O/Breakpad file
// to show basic metadata. This doesn't run during minidump parsing.
pub fn parse_dif_metadata(bytes: &[u8]) -> Result<Meta, String> {
    let obj = Object::parse(bytes).map_err(|e| format!("DIF parse: {e}"))?;

    Ok(Meta {
        kind: format!("{:?}", obj.kind()),
        arch: Some(obj.arch().to_string()),
        debug_id: Some(obj.debug_id().to_string()),
        code_id: obj.code_id().map(|c| c.to_string()),
    })
}
