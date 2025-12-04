# todo_cli_app

Simple command-line Todo list application written in Rust. Tasks are stored in a local `todos.txt` file in the project directory.

**Features**
- Add, list, mark complete, delete and clear completed tasks
- Small single-file storage format (`id|task|completed` per line)
- Lightweight and useful for learning Rust and building a tiny CLI

**Build & Run**
Requires the Rust toolchain (Cargo) on your PATH.

Build and run with Cargo:

```powershell
cd todo_cli_app
cargo run -- add "Buy milk and eggs"
cargo run -- list
```

Or build and run the binary directly:

```powershell
cd todo_cli_app
cargo build
.\target\debug\todo_cli_app.exe add "Buy milk"
.\target\debug\todo_cli_app.exe list
```

**Commands**
- `add <task>` — Add a new task (example: `todo add "Buy milk"`).
- `list` — List all tasks.
- `complete <id>` or `done <id>` — Mark task with id as completed.
- `delete <id>` — Delete a task by id.
- `clear` — Remove all completed tasks.
- `help` — Show usage help.

Examples:

```powershell
cargo run -- add "Walk the dog"
cargo run -- list
cargo run -- complete 1
cargo run -- clear
```

Storage
- Tasks are kept in `todos.txt` in the project folder. Each line uses the format `id|task|completed`.

PowerShell helper (optional)
- A helper script is provided at `scripts/todo.ps1` to make calling the binary easier from PowerShell. It exposes two functions when dot-sourced:
  - `Add-Task "..."` — builds the binary (if missing) and runs `add` with the text provided.
  - `todo <args>` — proxy that forwards any arguments to the built binary (e.g., `todo add "Buy milk"`).

Load the helper for the current session (PowerShell):

```powershell
. .\scripts\todo.ps1
Add-Task "Buy milk and eggs"
# or
todo add "Buy milk and eggs"
```

To make the helper persistent, add the dot-source line to your PowerShell profile (`$PROFILE`).

Notes
- The project is intentionally small and educational. If you want features like priority, due dates or a sync backend, open an issue or send a PR.
