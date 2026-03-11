use colored::*;

pub fn print_header(title: &str) {
    println!();
    println!("{}", "=".repeat(60).bright_blue());
    println!("  \u{1F4DA} {}", title.bold().bright_white());
    println!("{}", "=".repeat(60).bright_blue());
    println!();
}

pub fn print_section(title: &str) {
    println!("  {} {}", "\u{25B8}".bright_cyan(), title.bold());
}

pub fn print_content(text: &str) {
    for line in text.lines() {
        println!("    {}", line);
    }
    println!();
}

pub fn print_success(msg: &str) {
    println!("  {} {}", "\u{2713}".bright_green(), msg.green());
}

pub fn print_error(msg: &str) {
    println!("  {} {}", "\u{2717}".bright_red(), msg.red());
}

pub fn print_info(msg: &str) {
    println!("  \u{2139} {}", msg);
}

pub fn print_hint(msg: &str) {
    println!("  \u{1F4A1} {}", msg.yellow());
}

#[allow(dead_code)]
pub fn print_encouragement(correct: bool) {
    if correct {
        let msgs = ["Great job!", "Excellent!", "You got it!", "Perfect!", "Well done!"];
        let idx = rand::random::<usize>() % msgs.len();
        print_success(msgs[idx]);
    } else {
        let msgs = [
            "Not quite -- but mistakes help us learn!",
            "Keep trying -- you're getting closer!",
            "That's okay -- every expert was once a beginner!",
        ];
        let idx = rand::random::<usize>() % msgs.len();
        print_info(msgs[idx]);
    }
}

pub fn print_progress_bar(label: &str, value: f64, max: f64) {
    let pct = if max > 0.0 { value / max } else { 0.0 };
    let filled = (pct * 20.0).round() as usize;
    let empty = 20 - filled.min(20);
    let bar = format!(
        "{}{}",
        "\u{2588}".repeat(filled).bright_green(),
        "\u{2591}".repeat(empty).dimmed()
    );
    println!("  {} [{}] {:.0}%", label, bar, pct * 100.0);
}

pub fn print_divider() {
    println!("  {}", "\u{2500}".repeat(50).dimmed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_functions_dont_panic() {
        print_header("Test");
        print_section("Section");
        print_content("Hello\nWorld");
        print_success("Ok");
        print_error("Fail");
        print_info("Info");
        print_hint("Hint");
        print_encouragement(true);
        print_encouragement(false);
        print_progress_bar("Test", 50.0, 100.0);
        print_progress_bar("Zero", 0.0, 0.0);
        print_divider();
    }
}
