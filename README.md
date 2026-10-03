# Chess Project Beta

Welcome to the Chess Project! This project translates the core fundamentals of Rust into a three-tiered web and automation platform.

> **Note for Collaborators:** Enrique Sanchez, please make sure to read completely through this outline before beginning development!

---

##  How to Download and Run the Project

Follow these steps to pull the code cleanly from GitHub and get your local workspace running on your computer.

### 1. Clone the Repository
Open your terminal and clone the project:
```bash
git clone [https://github.com](https://github.com/AliPhilip05/ChessProject/)
cd ChessProject
```

### 2. Verify Your Sync
Make sure you are on the correct tracking branch and pull the latest changes:
```bash
git remote -v
git pull origin main
```

### 3. Run the Code Locally
To compile and test the project using Cargo, navigate to your root folder and execute:
```bash
cargo run
```

---

##  Project Purpose & Scope

The purpose of this project is to build an interactive chess application. The architecture is divided into three distinct sub-projects (\(P \subset \text{Project}\)) that build consecutively off the fundamentals taught in the introductory Rust documentation.

The overarching goal is to deploy a live webpage featuring three core functionalities:
1. **Guess the Elo Game:** A playable mode where users watch historical game moves and guess the specific skill rating (Elo) of the players.
2. **Player Stats Dashboard:** A live visual hub highlighting the developer's custom gameplay stats and performance metrics.
3. **Cloned Chess AI:** A personalized machine-learning or rule-based engine trained to mimic and replicate the exact playstyle of the developer, allowing users to test their skills against it.

---

##  Project Architecture

Below is the structured file hierarchy mapping out how the distinct components are isolated prior to final integration:

```text
chess_website_project/
├── main.rs                 # Project router & main gateway
│
├── guess_the_elo/          # Sub-Project 1: Interactive game engine
│   ├── main.rs
│   └── lib.rs
│
├── front_end_page/         # Sub-Project 2: Visual webpage assets
│   └── index.html
│
└── chess_ai_bot/           # Sub-Project 3: Playstyle cloning model
    ├── main.rs
    └── lib.rs
```

---

##  Sub-Project Roadmap

###  Phase 1: Guess The Elo (Chapter II)
Inspired directly by the standard "Guessing Game" archetype in the Rust manual. Rather than raw numbers, this iteration powers a custom tutorial and practice environment. Players interface via a terminal workflow allowing full session loops, menu switching, and graceful exit routines (`quit`).

###  Phase 2: Publication & Web Deployment (Chapter XXII)
The operational deployment of the front-end interfaces. This framework connects the user-facing web view directly to our backend Rust services, highlighting the live statistics dashboard alongside game logic wrappers.

###  Phase 3: Chess Bot AI
The development of a localized engine mimicking personal positional styles. Users on the frontend site will have the unique option to directly play against the bot to verify its behavioral mapping accuracy.

---

##  Contribution Workflow (For Programmers)

Always isolate your workspace tasks by spinning up dedicated features branches. Never commit directly to the `main` branch.

```bash
# 1. Create and switch to a targeted feature branch (e.g., menu)
git checkout -b feature/menu

# 2. Add and track the files you modified
git add src/lib.rs

# 3. Write a clear, descriptive commit message for code review
git commit -m "Add core navigation menu loops to guess_the_elo"

# 4. Push your feature branch securely up to GitHub
git push -u origin feature/menu
```
Once your feature branch compiles smoothly, open a **Pull Request (PR)** on GitHub to merge your work back into `main`.

