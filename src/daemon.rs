use std::path::PathBuf;
use std::time::Duration;
use tokio::{signal, task, time};
use daemonize::Daemonize;
use crate::{storage::StorageManager, logging::Logger};
use std::fs;

pub struct TrashDaemon {
    check_interval: Duration,
    logger: Logger,
}

impl TrashDaemon {
    pub fn new(check_interval: Duration) -> Result<Self, String> {
        let proj_dirs = directories::ProjectDirs::from("com", "larpi", "srm")
            .ok_or_else(|| "Cannot determine project directories".to_string())?;
        
        let log_path = proj_dirs.data_dir().join("daemon.log");
        let logger = Logger::new(log_path)?;

        Ok(Self { 
            check_interval,
            logger,
        })
    }

    pub async fn start(&mut self) -> Result<(), String> {
        self.logger.log("Starting trash monitoring daemon...")?;
        
        loop {
            tokio::select! {
                _ = self.check_trash() => {},
                _ = signal::ctrl_c() => {
                    self.logger.log("Received shutdown signal, stopping daemon...")?;
                    break;
                }
            }
            
            time::sleep(self.check_interval).await;
        }
        
        Ok(())
    }

    async fn check_trash(&mut self) -> Result<(), String> {
        let mut storage = StorageManager::new()?;
        if let Err(e) = storage.cleanup() {
            self.logger.log(&format!("Error during cleanup: {}", e))?;
        }
        Ok(())
    }
}

pub async fn start_daemon(interval: u64) -> Result<(), String> {
    let mut daemon = TrashDaemon::new(Duration::from_secs(interval))?;
    
    // Initialize daemon before starting tokio operations
    let proj_dirs = directories::ProjectDirs::from("com", "larpi", "srm")
        .ok_or_else(|| "Cannot determine project directories".to_string())?;
    let runtime_dir = proj_dirs.runtime_dir().unwrap_or(proj_dirs.data_dir());
    let pid_file = runtime_dir.join("daemon.pid");

    fs::create_dir_all(runtime_dir)
        .map_err(|e| format!("Failed to create runtime directory: {}", e))?;

    let daemonize = Daemonize::new()
        .pid_file(pid_file)
        .chown_pid_file(true)
        .working_directory(runtime_dir)
        .umask(0o027);

    daemonize.start().map_err(|e| format!("Error starting daemon: {}", e))?;
    
    // Now start the daemon loop
    daemon.start().await
} 