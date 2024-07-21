//! Download module.

use std::io::Write;
use std::path::Path;

/// Download a file to a given file.
pub fn download_url<U: reqwest::IntoUrl + std::fmt::Display>(
    url: U,
    output: &Path,
    sha256: Option<[u8; 32]>,
) -> anyhow::Result<usize> {
    use sha2::Digest;

    tracing::info!("Downloading {url} to {output:?}...");

    let client = reqwest::blocking::Client::new();

    // This will store the file in memory, shouldn't be a real concern since
    // it is only 15-20MB
    let resp = client
        .get(url)
        // The request gets rejected with no user-agent
        .header("User-Agent", "curl/8.1.2")
        .send()?;
    let _ = resp.error_for_status_ref()?;

    let bytes = resp.bytes()?;
    if let Some(sha256) = sha256 {
        let mut hasher = sha2::Sha256::new();
        hasher.update(&bytes);
        let hash: [u8; 32] = hasher.finalize().into();

        if sha256 != hash {
            anyhow::bail!("SHA256 mismatch!");
        }
    }

    let mut file = std::fs::File::create(output)?;
    file.write_all(&bytes)?;
    file.flush()?;

    Ok(bytes.len())
}
