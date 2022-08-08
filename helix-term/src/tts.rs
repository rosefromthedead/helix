use tokio::sync::mpsc::{Sender, channel};
use tts::Tts;

pub struct Utterance(pub String);

pub fn init_tts() -> Option<Sender<Utterance>> {
    let mut tts = if cfg!(target_os = "macos") {
        log::warn!("TTS is not supported yet on macOS");
        return None;
    } else {
        Tts::default()
            .map_err(|e| log::warn!("failed to initialize TTS: {e}"))
            .ok()?
    };
    let (tx, mut rx) = channel::<Utterance>(4);
    std::thread::spawn(move || {
        let res = tts.speak("Welcome to helix", true);
        if let Err(e) = res {
            log::warn!("tts failed: {e}");
        }
        while let Some(utterance) = rx.blocking_recv() {
            let res = tts.speak(&utterance.0, true);
            if let Err(e) = res {
                log::warn!("tts failed: {e}");
            }
        }
    });

    Some(tx)
}
