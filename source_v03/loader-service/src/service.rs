pub struct LoadService {
    srv_config: ServerConfig,
     app_config: AppConfig,
};

impl LoadService {
    pub async fn new(srv_config: ServerConfig, app_config: AppConfig) -> Result<Self, AppError> {
        Ok(LoadService {
            srv_config: srv_config,
            app_config: app_config,
        })
    }
}
