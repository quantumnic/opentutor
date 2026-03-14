mod commands;
mod db;
mod display;
mod engine;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "opentutor")]
#[command(about = "Offline AI personal tutor — world-class education for every child on earth.")]
#[command(version)]
struct Cli {
    /// Path to the database file
    #[arg(long, default_value = "opentutor.db")]
    db: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a learning session on a subject
    Learn {
        /// Subject to learn (e.g. "mathematics", "science")
        subject: String,
    },
    /// Take a quiz on a topic
    Quiz {
        /// Topic to quiz on (e.g. "fractions", "photosynthesis")
        topic: String,
        /// Number of questions
        #[arg(short, long, default_value = "5")]
        count: usize,
        /// Filter by difficulty (easy, medium, hard)
        #[arg(short, long)]
        difficulty: Option<String>,
    },
    /// Get a simple explanation of a concept
    Explain {
        /// Concept to explain (e.g. "photosynthesis", "gravity")
        concept: String,
    },
    /// Show your learning progress
    Progress,
    /// List available subjects
    Subjects,
    /// Show a learning path for a goal
    Path {
        /// Goal topic (e.g. "algebra", "cells")
        goal: String,
    },
    /// Review topics due for spaced repetition
    Review {
        /// Number of questions per topic
        #[arg(short, long, default_value = "3")]
        count: usize,
    },
    /// Show detailed learning statistics
    Stats,
    /// Search across all content (lessons, quizzes, explanations)
    Search {
        /// Search query
        query: String,
    },
    /// Show your personalized daily learning plan
    Daily,
    /// Reset your learning progress (optionally for a specific subject)
    Reset {
        /// Subject to reset (omit to reset everything)
        subject: Option<String>,
    },
    /// Export learning data as JSON
    Export {
        /// Output file path (prints to stdout if omitted)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Compare your progress across all subjects side-by-side
    Compare,
    /// Cross-topic challenge quiz mixing questions from multiple subjects
    Challenge {
        /// Number of questions
        #[arg(short, long, default_value = "10")]
        count: usize,
    },
    /// View your achievements and badges
    Achievements,
    /// Show a 7-day review forecast
    Forecast,
    /// Get personalized study recommendations based on your retention
    Recommend,
    /// Show leech cards (topics you're repeatedly struggling with)
    Leech,
    /// Quick one-screen summary of your learning state
    Summary,
    /// View your recent session activity history
    History {
        /// Number of entries to show
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// Show your learning streak and daily practice stats
    Streak,
    /// View or change configuration (retention target, daily goal, etc.)
    Config {
        /// Config key to view or set
        key: Option<String>,
        /// Value to set (omit to view current value)
        value: Option<String>,
    },
    /// Study flashcards for a topic or subject
    Flashcard {
        /// Topic or subject to study (e.g. "fractions", "history")
        topic: String,
        /// Number of flashcards to show
        #[arg(short, long, default_value = "10")]
        count: usize,
    },
    /// Bookmark (favorite) a topic for quick access
    Bookmark {
        /// Action: add, remove, list
        #[arg(default_value = "list")]
        action: String,
        /// Topic name (for add/remove)
        topic: Option<String>,
    },
    /// Show your weakest topics — where to focus study time
    Weak {
        /// Number of topics to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    /// Review your quiz mistakes to learn from errors
    Mistakes {
        /// Number of topics to show
        #[arg(short, long, default_value = "15")]
        limit: usize,
    },
    /// Mixed cross-subject quiz for interleaved practice
    Mix {
        /// Number of questions
        #[arg(short, long, default_value = "10")]
        count: usize,
        /// Filter by subject (optional)
        #[arg(short, long)]
        subject: Option<String>,
    },
    /// Track your learning velocity and progress trends
    Velocity,
    /// Show optimal study focus based on forgetting curve analysis
    Focus,
    /// Look up terms across all subjects (glossary)
    Glossary {
        /// Term to search (lists all if omitted)
        term: Option<String>,
    },
    /// Show a study activity heatmap (like GitHub contributions)
    Heatmap {
        /// Number of weeks to display
        #[arg(short, long, default_value = "12")]
        weeks: usize,
    },
    /// Show your best study hours based on performance data
    BestHours,
    /// Show performance trends over recent days
    Trend {
        /// Number of days to show
        #[arg(short, long, default_value = "14")]
        days: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    let conn = match db::init_db(&cli.db) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error opening database: {}", e);
            std::process::exit(1);
        }
    };

    let result = match cli.command {
        Commands::Learn { subject } => commands::learn::run(&conn, &subject),
        Commands::Quiz { topic, count, difficulty } => commands::quiz::run(&conn, &topic, count, difficulty.as_deref()),
        Commands::Explain { concept } => commands::explain::run(&conn, &concept),
        Commands::Progress => commands::progress::run(&conn),
        Commands::Subjects => commands::subjects::run(&conn),
        Commands::Path { goal } => commands::path::run(&conn, &goal),
        Commands::Review { count } => commands::review::run(&conn, count),
        Commands::Stats => commands::stats::run(&conn),
        Commands::Search { query } => commands::search::run(&conn, &query),
        Commands::Daily => commands::daily::run(&conn),
        Commands::Reset { subject } => commands::reset::run(&conn, &subject),
        Commands::Export { output } => commands::export::run(&conn, &output),
        Commands::Compare => commands::compare::run(&conn),
        Commands::Challenge { count } => commands::challenge::run(&conn, count),
        Commands::Achievements => commands::achievements::run(&conn),
        Commands::Forecast => commands::forecast::run(&conn),
        Commands::Recommend => commands::recommend::run(&conn),
        Commands::Leech => commands::leech::run(&conn),
        Commands::Summary => commands::summary::run(&conn),
        Commands::History { limit } => commands::history::run(&conn, limit),
        Commands::Streak => commands::streak::run(&conn),
        Commands::Config { key, value } => commands::config::run(&conn, &key, &value),
        Commands::Flashcard { topic, count } => commands::flashcard::run(&conn, &topic, count),
        Commands::Bookmark { action, topic } => commands::bookmark::run(&conn, &action, &topic),
        Commands::Weak { limit } => commands::weak::run(&conn, limit),
        Commands::Mistakes { limit } => commands::mistakes::run(&conn, limit),
        Commands::Mix { count, subject } => commands::mix::run(&conn, count, subject.as_deref()),
        Commands::Velocity => commands::velocity::run(&conn),
        Commands::Focus => commands::focus::run(&conn),
        Commands::Glossary { term } => commands::glossary::run(&conn, &term),
        Commands::Heatmap { weeks } => commands::heatmap::run(&conn, weeks),
        Commands::BestHours => commands::best_hours::run(&conn),
        Commands::Trend { days } => commands::trend::run(&conn, days),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
