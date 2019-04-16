use abscissa::Callable;

#[derive(Debug, Options)]
pub struct BackupCommand {
    #[options(short = "c", long = "config")]
    pub config: Option<String>,

    #[options(short = "v", long = "verbose")]
    pub verbose: bool,
}

impl Default for BackupCommand {
    fn default() -> Self {
        Self {
            config: None,
            verbose: false,
        }
    }
}

impl Callable for BackupCommand {
    fn call(&self) {}
}
