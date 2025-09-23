use crate::structs::StatsFile;
use anyhow::{Result, bail};
use notify::{
    Config,
    EventKind::Access,
    RecommendedWatcher, RecursiveMode, Watcher,
    event::{AccessKind::Close, AccessMode},
};
use std::path::Path;
use tokio::{fs, sync::mpsc};
pub async fn main(world_path: String) -> Result<()> {
    let handle = tokio::runtime::Handle::current();
    let (tx, mut rx) = mpsc::channel(100);

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            let tx = tx.clone();
            handle.spawn(async move {
                if tx.send(res).await.is_err() {
                    eprintln!("Receiver dropped");
                }
            });
        },
        Config::default(),
    )
    .unwrap();
    watcher
        .watch(
            &Path::new(&world_path).join("stats/"),
            RecursiveMode::Recursive,
        )
        .expect("Error watching");
    while let Some(event) = rx.recv().await {
        match event {
            Ok(ev) => match ev.kind {
                Access(Close(AccessMode::Write)) => {
                    let file = fs::read_to_string(
                        Path::new(&world_path)
                            .join("stats/")
                            .join(ev.paths[0].clone()),
                    )
                    .await?;
                    let stats_file: StatsFile =
                        serde_json::from_str(&file).unwrap();

                    println!("{:?}", ev);
                }
                _ => {}
            },
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
    Ok(())
}
