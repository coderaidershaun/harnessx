//! CLI entry point for harnessx project management.

use clap::{Parser, Subcommand};

use harnessx::commands::context::ContextCommand;
use harnessx::commands::init::InitArgs;
use harnessx::commands::intake_completion::IntakeCompletionCommand;
use harnessx::commands::intake_onboarding::IntakeOnboardingCommand;
use harnessx::commands::intake_actions::IntakeActionsCommand;
use harnessx::commands::intake_team::IntakeTeamCommand;
use harnessx::commands::planning::PlanningCommand;
use harnessx::commands::planning_epics::PlanningEpicsCommand;
use harnessx::commands::planning_milestones::PlanningMilestonesCommand;
use harnessx::commands::planning_stories::PlanningStoriesCommand;
use harnessx::commands::planning_tasks::PlanningTasksCommand;
use harnessx::commands::progress::ProgressCommand;
use harnessx::commands::project::ProjectCommand;

#[derive(Parser)]
#[command(name = "harnessx", version, about = "CLI for harnessx project management")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Search for tags and wikilinks across markdown files.
    Context {
        #[command(subcommand)]
        command: ContextCommand,
    },
    /// Scaffold the harnessx harness in the current directory.
    Init(InitArgs),
    /// Manage projects.
    Project {
        #[command(subcommand)]
        command: ProjectCommand,
    },
    /// Manage intake onboarding progress.
    IntakeOnboarding {
        #[command(subcommand)]
        command: IntakeOnboardingCommand,
    },
    /// Manage intake completion progress.
    IntakeCompletion {
        #[command(subcommand)]
        command: IntakeCompletionCommand,
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
    /// Manage intake team progress.
    IntakeTeam {
        #[command(subcommand)]
        command: IntakeTeamCommand,
    },
    /// Manage planning stage progress.
    Planning {
        #[command(subcommand)]
        command: PlanningCommand,
    },
    /// Manage planning epics.
    PlanningEpics {
        #[command(subcommand)]
        command: PlanningEpicsCommand,
    },
    /// Manage planning milestones.
    PlanningMilestones {
        #[command(subcommand)]
        command: PlanningMilestonesCommand,
    },
    /// Manage planning stories.
    PlanningStories {
        #[command(subcommand)]
        command: PlanningStoriesCommand,
    },
    /// Manage planning tasks.
    PlanningTasks {
        #[command(subcommand)]
        command: PlanningTasksCommand,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Context { command } => command.run(),
        Command::Init(args) => args.run(),
        Command::Project { command } => command.run(),
        Command::IntakeOnboarding { command } => command.run(),
        Command::IntakeCompletion { command } => command.run(),
        Command::Progress { command } => command.run(),
        Command::IntakeActions { command } => command.run(),
        Command::IntakeTeam { command } => command.run(),
        Command::Planning { command } => command.run(),
        Command::PlanningEpics { command } => command.run(),
        Command::PlanningMilestones { command } => command.run(),
        Command::PlanningStories { command } => command.run(),
        Command::PlanningTasks { command } => command.run(),
    }
}
