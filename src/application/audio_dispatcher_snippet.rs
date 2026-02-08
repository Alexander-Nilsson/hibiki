pub fn new() -> Result<(Self, AudioDispatcher)> {
    let state = Arc::new(RwLock::new(AudioState {
        pack: None,
        volume: 1.0,
        enabled: false,
        pack_name: None,
    }));

    let (cmd_tx, cmd_rx) = std::sync::mpsc::channel::<PlayCommand>();

    std::thread::spawn(move || {
        tracing::info!("Audio thread started");
    });

    let dispatcher = AudioDispatcher {
        state: state.clone(),
        cmd_tx,
    };

    Ok((Self { state }, dispatcher))
}
