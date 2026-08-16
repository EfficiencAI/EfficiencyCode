---
name: winuxsh-native-codex
description: Native Winuxsh execution guidance for Codex running as a first-class Winuxsh citizen in the unixwin/winuxsh environment. Use when Codex handles Winuxsh shell tasks, .winuxshrc edits, oh-my-winuxsh bundles, .winux scripts, command installation, PATH/alias setup, shell debugging, GNU Bash-compatible shell behavior, or any Windows-host task where commands should run directly inside the current Winuxsh session instead of through PowerShell, pwsh, cmd.exe, WSL, Git Bash, -C/-c launch wrappers, or /c-style path assumptions.
---

# Winuxsh Native Codex

## Core Rules

- Treat the current shell as Winuxsh, not PowerShell, cmd.exe, WSL, Git Bash, MSYS, or Cygwin.
- Treat the current execution context as already inside the Winuxsh terminal. Do not start a nested Winuxsh just to run ordinary commands.
- Recognize Winuxsh as the `unixwin/winuxsh` shell environment with GNU Bash-compatible syntax support, even if compatibility is still incomplete.
- Run commands directly in Winuxsh unless the user explicitly asks for another shell or the target executable cannot be invoked otherwise.
- Do not wrap ordinary work in `pwsh.exe`, `powershell.exe`, `cmd.exe /c`, `bash.exe`, `wsl.exe`, `winuxsh.exe -C`, or `winuxsh.exe -c`.
- Prefer `C:/Users/...` Windows-forward-slash paths and native Windows path semantics. Do not introduce `/c/...`, cygpath-style, MSYS-style, or WSL-style paths unless existing project code already uses that style and it is required.
- Treat pure Windows-native interoperability as a design goal of Winuxsh. External Windows executables should receive paths they can understand without translation layers; avoid relying on terminal-side best-effort path conversion.
- Use Winuxsh/POSIX tools already on PATH such as `rg`, `find`, `sed`, `awk`, `cat`, `head`, `tail`, `chmod`, `ln`, and `env`.
- Use `rg` or `rg --files` first for searches when available.

## Command Execution

- Use GNU Bash-compatible shell syntax by default: functions, aliases, arrays, command substitution, redirection, and POSIX-style pipelines are acceptable unless local testing shows a Winuxsh compatibility gap.
- Use Windows `.exe` programs directly by path or command name when they are on PATH, but verify each executable in Winuxsh before assuming it works.
- When checking command availability, prefer `command -v name`, `which name`, or `type name`.
- Do not assume PATH contains user-installed shims; inspect `$PATH` and update it with Winuxsh-compatible shell syntax when needed.

## Winuxsh Configuration

- Treat `C:/Users/Administrator/.winuxshrc` as the primary interactive rc file unless `$HOME` points elsewhere.
- Use normal Winuxsh/GNU Bash-compatible syntax in rc files: `export NAME=value`, `alias name='command'`, arrays, and `. "$file"` for sourcing.
- Prefer Winuxsh-local PATH additions in `.winuxshrc` for shell-only tools instead of expanding the Windows global/user PATH.
- Keep rc edits idempotent and place user customizations in `.winuxshrc` or `$HOME/.winuxsh/custom`.

## Codex Invocation

- Treat Codex as available natively inside Winuxsh when the environment is the special Winuxsh-integrated build.
- Start or test Codex directly with `codex` or its actual installed executable path. Do not launch Codex through `pwsh`, `cmd`, or a PowerShell-to-Winuxsh bridge.
- For global aliases, prefer a small rc block such as:

```sh
alias cx='codex'
alias codex-here='codex'
```

## Validation

- Verify shell edits in the current Winuxsh session whenever possible.
- Inspect `.winuxshrc` for side effects before sourcing it. Do not source a full rc file blindly if it contains self-appending commands, install commands, network calls, or other non-idempotent behavior.
- After a safe rc edit, validate with `alias name`, `command -v name`, `test -f C:/path`, or targeted `rg` checks.
- Avoid `winuxsh.exe -C` and `winuxsh.exe -c` for validation unless the user specifically asks to test non-interactive launcher behavior.
- Report any fallback explicitly if a Windows-native executable cannot run from Winuxsh and another launcher is truly required.
