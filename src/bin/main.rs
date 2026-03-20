//! CLI entry point for harnessx project management.

use clap::{Parser, Subcommand};

use harnessx::commands::init::InitArgs;
use harnessx::commands::intake::IntakeCommand;
use harnessx::commands::intake_actions::IntakeActionsCommand;
use harnessx::commands::progress::ProgressCommand;
use harnessx::commands::project::ProjectCommand;

#[derive(Parser)]
#[command(name = "harnessx", about = "CLI for harnessx project management")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Scaffold the harnessx harness in the current directory.
    Init(InitArgs),
    /// Manage projects.
    Project {
        #[command(subcommand)]
        command: ProjectCommand,
    },
    /// Manage intake progress.
    Intake {
        #[command(subcommand)]
        command: IntakeCommand,
    },
    /// Manage project pipeline progress.
    Progress {
        #[command(subcommand)]
        command: ProgressCommand,
    },
    /// Manage intake action items.
    IntakeActions {
        #[command(subcommand)]
        command: IntakeActionsCommand,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Init(args) => args.run(),
        Command::Project { command } => command.run(),
        Command::Intake { command } => command.run(),
        Command::Progress { command } => command.run(),
        Command::IntakeActions { command } => command.run(),
    }
}
