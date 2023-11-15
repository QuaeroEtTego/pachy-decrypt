use std::{error::Error, sync::Arc};

use tokio::task::JoinSet;

use keyset::KeySet;
use pachy::Pachy;

mod keyset;
mod pachy;

type ForensicResult<T> = Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> ForensicResult<()> {
    let key_sets = Arc::new(KeySet::load()?);
    let pachy_files = Pachy::from_files()?;

    let mut join_set = JoinSet::new();

    for pachy in pachy_files.into_iter() {
        let key_sets_clone = key_sets.clone();

        // Spawn a task for each .pachy file
        join_set.spawn(async move {
            if let Err(e) = pachy.decrypt(key_sets_clone) {
                eprintln!("Error while trying to decrypt a pachy file: {e}.");
            }
        });
    }

    // Wait for all tasks to finish
    while (join_set.join_next().await).is_some() {}

    Ok(())
}
