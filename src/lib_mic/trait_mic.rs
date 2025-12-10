pub(crate) trait MicrophoneT {
    fn new() -> anyhow::Result<Self>
    where
        Self: Sized;

    async fn boom_pattern(&self) -> anyhow::Result<()>;
    
    fn spawn_error_pattern(&self);
}