use std::{io, path::Path};

use crate::fs::opfs::open_dir;

pub async fn create_dir(path: impl AsRef<Path>) -> io::Result<()> {
    open_dir(path, true, false).await?;
    Ok(())
}
