use tokio::time::sleep;

use super::models::ERROR_START;

#[allow(dead_code)]
pub(crate) struct MicrophoneMock {}

impl MicrophoneMock {
    pub(crate) fn new() -> anyhow::Result<Self> {
        println!("mic: new");
        Ok(Self {})
    }

    #[allow(dead_code)]
    pub(crate) async fn boom_pattern(&self) -> anyhow::Result<()> {
        println!("mic: boom pattern");
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn spawn_error_pattern(&self) {
        ERROR_START.call_once(|| {
            tokio::spawn(async move {
                loop {
                    println!("mic: error pattern output");
                    sleep(std::time::Duration::from_secs(1)).await;
                }
            });
        });
    }
}