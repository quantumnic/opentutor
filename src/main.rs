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
        Commands::Quiz { topic, count } => commands::quiz::run(&conn, &topic, count),
        Commands::Explain { concept } => commands::explain::run(&conn, &concept),
        Commands::Progress => commands::progress::run(&conn),
        Commands::Subjects => commands::subjects::run(&conn),
        Commands::Path { goal } => commands::path::run(&conn, &goal),
        Commands::Review { count } => commands::review::run(&conn, count),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
