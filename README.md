# 📚 opentutor

**World-class education for every child on earth.**

An offline-first, free, personal AI tutor that adapts to your learning pace. No internet required. No subscription. No data collection. Just learning.

---

## 🌍 Mission

Every child deserves access to a patient, knowledgeable tutor — regardless of where they live, what language they speak, or what their family can afford. **opentutor** is our answer: a free, offline-capable personal tutor that runs on any computer.

## ✨ Features

- **Adaptive Learning** — Tracks what you know and what you need to review
- **Spaced Repetition** — Uses the SM-2 algorithm to optimize review timing
- **Multiple Subjects** — Mathematics, Science, Language, History, Health, Computer Science, Geography
- **Quiz System** — Multiple-choice and true/false questions with hints and explanations
- **Review Command** — Dedicated spaced repetition review for due topics
- **Learning Paths** — Step-by-step guides toward mastery goals
- **Offline-First** — Everything stored locally in SQLite, no internet needed
- **Free Forever** — Open source, no ads, no tracking

## 🚀 Quick Start

```bash
# Install from source
git clone https://github.com/quantumnic/opentutor
cd opentutor
cargo install --path .

# Start learning!
opentutor subjects                    # See what's available
opentutor learn "mathematics"         # Start a learning session
opentutor quiz "fractions"            # Test your knowledge
opentutor explain "photosynthesis"    # Get a simple explanation
opentutor progress                    # Check your progress
opentutor path "algebra"              # See a learning path
opentutor review                      # Review due topics (spaced repetition)
```

## 📖 Commands

| Command | Description |
|---------|-------------|
| `opentutor learn <subject>` | Start a learning session on a subject |
| `opentutor quiz <topic>` | Take a quiz on a specific topic |
| `opentutor explain <concept>` | Get a beginner-friendly explanation |
| `opentutor progress` | View your learning progress and stats |
| `opentutor subjects` | List all available subjects and topics |
| `opentutor path <goal>` | Show a step-by-step learning path |
| `opentutor review` | Review due topics using spaced repetition |

## 📚 Subjects & Topics

### Mathematics
- Arithmetic (addition, subtraction, multiplication, division)
- Fractions (proper, improper, mixed numbers)
- Percentages (conversions, calculations)
- Algebra Basics (variables, equations)
- Geometry (shapes, angles, area)

### Science
- Photosynthesis (how plants make food)
- Cell Division (mitosis)
- Gravity (forces and acceleration)
- States of Matter (solids, liquids, gases)

### Language
- Grammar Basics (parts of speech)
- Reading Comprehension (active reading strategies)

### History
- Ancient Civilizations (Mesopotamia, Egypt, Indus Valley, China)
- World Wars (WWI & WWII overview)
- Industrial Revolution (inventions, societal change)

### Health
- Hygiene (hand washing, disease prevention)
- Nutrition (food groups, healthy eating)
- First Aid Basics (cuts, burns, choking)

### Computer Science
- Binary & Data (number systems, data representation)
- Algorithms (searching, sorting, Big O)
- Programming Concepts (variables, loops, functions, debugging)
- Computer Hardware (CPU, RAM, storage, I/O)

### Geography
- Continents & Oceans (landmasses, major features)
- Weather & Climate (climate zones, water cycle)
- Maps & Navigation (reading maps, latitude/longitude)
- Natural Resources (renewable vs non-renewable)

## 🧠 How It Works

### Adaptive Learning
opentutor tracks your performance on each topic. As you answer questions correctly, the difficulty adapts:
- **Beginner** → foundational concepts
- **Intermediate** → deeper understanding
- **Advanced** → mastery-level challenges

### Spaced Repetition
Using a simplified SM-2 algorithm (the same principle behind Anki), opentutor schedules reviews at optimal intervals. Topics you struggle with come back sooner; topics you've mastered appear less frequently.

### Socratic Method
Rather than just presenting facts, opentutor asks follow-up questions to encourage deeper thinking. The goal is understanding, not memorization.

## 🛠️ Tech Stack

- **Rust** — Fast, safe, compiles to a single binary
- **SQLite** (via rusqlite) — Zero-config embedded database
- **clap** — CLI argument parsing
- **colored** — Beautiful terminal output
- **rand** — Quiz randomization
- **serde** — Serialization

## 🤝 Contributing

Contributions welcome! Areas where help is needed:

- **More subjects and content** — Add topics, lessons, quiz questions
- **Translations** — Help make opentutor available in every language
- **Interactive mode** — stdin-based quiz answering
- **AI integration** — Optional LLM-powered explanations
- **Accessibility** — Screen reader support, simplified output

```bash
# Run tests
cargo test

# Run clippy
cargo clippy -- -D warnings

# Build release
cargo build --release
```

## 📄 License

MIT — Free as in freedom. Use it, modify it, share it.

---

*"Education is the most powerful weapon which you can use to change the world." — Nelson Mandela*
