use clap::{Args, Parser, Subcommand};

// ========================== Slurmlet ==========================
#[derive(Debug, Parser, PartialEq)]
#[command(name = "slurmlet", author, version = version(), about = "A tiny job scheduler inspired by Slurm.")]
pub struct JobArgs {
    /// Sub Commands
    #[command(subcommand)]
    pub commands: Option<Commands>,
}

impl JobArgs {
    pub fn new() -> Self {
        Self::parse()
    }
}

/// CLI commands
#[derive(Debug, Subcommand, PartialEq)]
pub enum Commands {
    /// Submit tasks
    Submit(SubmitArgs),
    /// Job status
    Status(StatusArgs),
    /// Cancel job
    Cancel(CancelArgs),
    /// List jobs
    List(ListArgs),
    /// View job logs
    Log(LogArgs),
    /// Update job priority
    Priority(PriorityArgs),
    /// Pause job
    Hold(HoldArgs),
    /// Resume job
    Resume(ResumeArgs),
    /// View system or job information
    Info(InfoArgs),
    /// Start the slurmlet daemon
    Start(StartArgs),
    /// Stop the slurmlet daemon
    Stop(StopArgs),
    /// Restart the slurmlet daemon
    Restart(RestartArgs),
}

/// Arguments for the `submit` command
#[derive(Debug, clap::Args, PartialEq)]
pub struct SubmitArgs {
    /// Path to the script file
    #[arg(short, long, required_unless_present = "command")]
    pub file: Option<String>,

    /// Command to submit as a task
    #[arg(short, long, conflicts_with = "file")]
    pub command: Option<String>,

    /// Task priority (e.g., low, normal, high)
    #[arg(short, long, default_value = "normal")]
    pub priority: String,
}

/// Arguments for the `status` command
#[derive(Debug, clap::Args, PartialEq)]
pub struct StatusArgs {
    /// Job ID to check the status of
    #[arg(value_name = "JOB_ID", required = false)]
    pub job_id: Option<u32>,

    /// Show status of all jobs
    #[arg(short, long)]
    pub all: bool,

    /// Filter jobs by state (e.g., pending, running, completed)
    #[arg(short, long, value_name = "STATE")]
    pub state: Option<String>,
}

/// Arguments for the `cancel` command
#[derive(Debug, clap::Args, PartialEq)]
pub struct CancelArgs {
    /// Job ID to cancel
    #[arg(value_name = "JOB_ID", required_unless_present = "all")]
    pub job_id: Option<u32>,

    /// Cancel all jobs
    #[arg(short, long, conflicts_with = "job_id")]
    pub all: bool,
}

/// Arguments for the `list` command
#[derive(Debug, clap::Args, PartialEq)]
pub struct ListArgs {
    /// Show jobs for a specific user
    #[arg(short, long, value_name = "USER")]
    pub user: Option<String>,

    /// Show jobs with a specific state
    #[arg(short, long, value_name = "STATE")]
    pub state: Option<String>,

    /// Show all jobs (regardless of user)
    #[arg(short, long)]
    pub all: bool,
}

/// Arguments for the `log` command
#[derive(Debug, clap::Args, PartialEq)]
pub struct LogArgs {
    /// Job ID to view logs for
    #[arg(value_name = "JOB_ID", required = true)]
    pub job_id: u32,

    /// Follow logs (like `tail -f`)
    #[arg(short, long)]
    pub follow: bool,
}

/// Arguments for the `priority` command
#[derive(Debug, clap::Args, PartialEq)]
pub struct PriorityArgs {
    /// Job ID to change the priority of
    #[arg(value_name = "JOB_ID", required = true)]
    pub job_id: u32,

    /// New priority level (e.g., low, normal, high)
    #[arg(value_name = "PRIORITY", required = true)]
    pub priority: String,
}

/// Arguments for the `hold` command
#[derive(Debug, clap::Args, PartialEq)]
pub struct HoldArgs {
    /// Job ID to hold
    #[arg(value_name = "JOB_ID", required = true)]
    pub job_id: u32,
}

/// Arguments for the `resume` command
#[derive(Debug, clap::Args, PartialEq)]
pub struct ResumeArgs {
    /// Job ID to resume
    #[arg(value_name = "JOB_ID", required = true)]
    pub job_id: u32,
}

/// Arguments for the `info` command
#[derive(Debug, clap::Args, PartialEq)]
pub struct InfoArgs {
    /// Show information about running jobs
    #[arg(short, long)]
    pub jobs: bool,

    /// Show system information
    #[arg(short, long)]
    pub system: bool,
}

/// Arguments for the `start` command
#[derive(Debug, clap::Args, PartialEq)]
pub struct StartArgs {
    /// Run the slurmlet daemon in the background
    #[arg(short, long)]
    pub daemon: bool,
}

/// Arguments for the `stop` command
#[derive(Debug, clap::Args, PartialEq)]
pub struct StopArgs {
    /// Force stop the daemon
    #[arg(short, long)]
    pub force: bool,
}

/// Arguments for the `restart` command
#[derive(Debug, clap::Args, PartialEq)]
pub struct RestartArgs {}

// ========================== Daemon ==========================
#[derive(Debug, Parser, PartialEq)]
#[command(author, version = version(), about)]
pub struct DaemonArgs {
    /// Config file path.
    /// Default: $HOME/.slurmlet/config.toml
    #[arg(short, long)]
    pub config: Option<String>,

    /// Load cached tasks
    #[arg(short, long, default_value = "true")]
    pub load: bool,
}

impl DaemonArgs {
    pub fn new() -> Self {
        Self::parse()
    }
}

const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "-",
    env!("VERGEN_GIT_DESCRIBE"),
    " (",
    env!("VERGEN_BUILD_DATE"),
    ")"
);

pub fn version() -> &'static str {
    let author = clap::crate_authors!();

    Box::leak(Box::new(format!(
        "\
{VERSION_MESSAGE}

Authors: {author}"
    )))
}