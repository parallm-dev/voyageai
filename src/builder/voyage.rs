use crate::client::VoyageAiClient;
use crate::config::VoyageConfig;

pub struct VoyageBuilder {
    config: Option<VoyageConfig>,
}

impl Default for VoyageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl VoyageBuilder {
    pub fn new() -> Self {
        VoyageBuilder { config: None }
    }

    pub fn with_config(mut self, config: VoyageConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> Result<VoyageAiClient, &'static str> {
        let config = self.config.ok_or("Config is required")?;
        Ok(VoyageAiClient::new(config))
    }
}
