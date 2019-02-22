use stremio_state_ng::types::*;

const MANIFEST_RAW: &str = include_str!("./manifest.json");

fn main() {
    let _manifest: Manifest = serde_json::from_str(MANIFEST_RAW)
        .expect("failed to parse manifest: invalid manifest");
}
