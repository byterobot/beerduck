pub fn listen_modified() -> Result<RecommendedWatcher, Error> {
    let mut watcher = RecommendedWatcher::new(|e: Result<Event, notify::Error>| {
        if let Ok(event) = e {
            // notify web
        }
    }, notify::Config::default())?;
    watcher.watch(&workspace().posts, RecursiveMode::Recursive)?;
    watcher.watch(&workspace().theme.self_dir, RecursiveMode::Recursive)?;
    watcher.watch(&workspace().assets.images, RecursiveMode::Recursive)?;
    Ok(watcher)
}