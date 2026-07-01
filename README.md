# minish

A minimal Unix shell written in Rust. It reads commands from a prompt, parses them into pipelines, and executes external programs with support for pipes, I/O redirection, and a few builtins.

## Quick start

```bash
cargo build --release
cargo run
```

You get an interactive REPL:

```
minish > echo hello world
minish > ls -l | wc -l
minish > sort < names.txt > sorted.txt
minish > cd /tmp
minish > exit
```

## How it works

minish follows the classic shell pipeline: **read → tokenize → parse → execute**. Each stage is a separate module with a single responsibility.

```
┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐
│  REPL   │───▶│  lexer  │───▶│ parser  │───▶│  exec   │
│ main.rs │    │ tokens  │    │Pipeline │    │ spawn   │
└─────────┘    └─────────┘    └─────────┘    └─────────┘
```

### 1. REPL (`main.rs`)

The main loop prints a prompt, reads a line from stdin, and passes it through the pipeline. It keeps running until you type `exit`.

### 2. Lexer (`lexer.rs`)

The lexer turns a raw input string into a stream of tokens. It is a pure function — no side effects, easy to unit test.

**Token types:**

| Token | Example | Meaning |
|-------|---------|---------|
| `Word` | `ls`, `hello world` | A command name or argument |
| `Pipe` | `\|` | Connects two commands |
| `InputRedirect` | `<` | Read stdin from a file |
| `OutputRedirect` | `>` | Write stdout to a file |

**Quoting:** Double quotes (`"..."`) and single quotes (`'...'`) group words together. Whitespace inside quotes is preserved. Operators inside quotes are treated as literal characters.

```
echo "hello world"   →  [Word("echo"), Word("hello world")]
ls | wc              →  [Word("ls"), Pipe, Word("wc")]
sort < in > out      →  [Word("sort"), InputRedirect, Word("in"), OutputRedirect, Word("out")]
```

Unclosed quotes produce an error instead of a panic.

### 3. Parser (`parser.rs`)

The parser converts tokens into a `Pipeline` — a list of `Cmd` structs. It also does not spawn anything.

```rust
pub struct Pipeline {
    pub commands: Vec<Cmd>,
}

pub struct Cmd {
    pub argv: Vec<String>,              // argv[0] is the program name
    pub stdin_from: Option<String>,     // < file
    pub stdout_to: Option<String>,     // > file
}
```

**Grammar:**

```
pipeline := command ( "|" command )*
command  := word+ redirection*
```

Examples:

| Input | Parsed structure |
|-------|-----------------|
| `ls \| wc` | Two commands: `ls` and `wc` |
| `sort < names.txt > out.txt` | One command with stdin and stdout redirected |
| `echo hello` | One command with two arguments |

The parser rejects malformed input (e.g. `ls \| \| wc`, missing filenames after `<` or `>`).

### 4. Executor (`exec.rs`)

The executor is the only module that touches the OS. It takes a `Pipeline` and runs it using `std::process::Command`.

#### Single commands

For a command with no pipes, the executor handles three cases:

1. **Builtin `cd`** — runs in-process via `std::env::set_current_dir` (see [Why is `cd` a builtin?](#why-is-cd-a-builtin))
2. **Redirection** — opens files and wires them to the child process's stdin/stdout
3. **Plain execution** — spawns the program and waits for it to finish

#### Pipelines

For `cmd1 | cmd2 | cmd3`, all stages are spawned **concurrently**:

1. Spawn `cmd1` with a piped stdout
2. Take that pipe and connect it to `cmd2`'s stdin
3. Repeat for each stage
4. Wait for all children to finish

This matters for commands like `yes | head -3` — if stages ran sequentially, `yes` would fill a buffer and block forever.

#### Redirection

| Operator | Behavior |
|----------|----------|
| `< file` | Opens `file` and feeds it as the command's stdin |
| `> file` | Creates/truncates `file` and writes the command's stdout to it |

Redirection and pipes can be combined on a single command (e.g. `sort < in.txt > out.txt`).

## Why is `cd` a builtin?

When you run `cd /tmp`, you expect the **shell itself** to change its working directory — not a child process.

If `cd` were an external program, the OS would fork a child, the child would call `chdir("/tmp")`, and then exit. The parent shell would still be in the original directory. Every subsequent command would run from the wrong place.

Builtins like `cd`, `pwd`, and `exit` must run **inside the shell process** because they modify shell state (current directory, whether the REPL continues). External programs always run as separate child processes and cannot change the parent's environment.

## Project structure

```
src/
├── main.rs    # REPL loop
├── lexer.rs   # String → tokens (pure, unit-tested)
├── parser.rs  # Tokens → Pipeline struct (pure)
└── exec.rs    # Pipeline → spawned processes (only module that uses std::process)
```

## Dependencies

None. minish uses only the Rust standard library (`std::process`, `std::env`, `std::fs`, `std::io`).

## Running tests

The lexer includes unit tests for tokenization:

```bash
cargo test
```

## Current limitations

- **`cd` only** — `pwd` and `exit` are recognized at the REPL level but not fully implemented as builtins in the executor
- **No `-c` flag** — runs interactively only (no `minish -c "command"` one-shot mode yet)
- **No append redirection** — `>>` is not supported
- **No chaining** — `&&`, `||`, and `;` are not supported
- **No variable expansion** — `$VAR` and `$?` are not supported
- **No globbing** — `*.txt` is passed literally to the program

## Example session

```
minish > echo one two   three
Pipeline { commands: [Cmd { argv: ["echo", "one", "two", "three"], stdin_from: None, stdout_to: None }] }

minish > ls | wc -l
Pipeline { commands: [Cmd { argv: ["ls"], ... }, Cmd { argv: ["wc", "-l"], ... }] }

minish > sort < file.txt > sorted.txt
Pipeline { commands: [Cmd { argv: ["sort"], stdin_from: Some("file.txt"), stdout_to: Some("sorted.txt") }] }

minish > cd /home/user
minish > exit
```
