# Introduction

The GTE Book is a guided reading of the Rust code behind GTE, a chess-game practice app with a terminal curriculum. Its purpose is broader than explaining how to click through the two game modes: it follows the data, state, and tests through the complete repository.

## Start at Question 0

The terminal opens with the GTE mark and Question 0. That exercise is intentionally filled in. Run its check, read the two passing test names, and use the component map to find the real source files. You can start with [Question 0: Start with GTE](/guide/question-zero).

After that, each practice item isolates one behavior from the app. Its `NOTE:` explains the expected behavior, its `TODO:` identifies the code to complete, and its test names the behavior being checked. Every exercise links to the matching section in the complete [Codebase Walkthrough — reference.md](/guide/reference).

## What is in this book

The book begins with Cargo packages and Rust types, then follows a game from a public Chess.com archive through PGN parsing and board snapshots. It explains how the web server handles requests, stores a game session, renders pages, and scores answers. It also covers the terminal interface, progress tracking, focused exercise tests, solutions, and capstone.

Read it in order to follow the data path, or use the chapter list to jump directly to a module. The reference chapter includes the source files, helper functions, tests, and known limits so that you can trace any exercise back to the running app.

## Requirements

You need Rust and Cargo to run the application and its tests. The built-in demo game works offline. Public archive challenges need a network connection but do not require a Chess.com login. The exercises are local and do not make API requests.

Continue to [Getting Started](/guide/getting-started) for commands, or open the [project map](/guide/architecture) before reading the implementation chapters.
