# ts_deplint

A tool to lint dependencies within TypeScript projects.

## Usage

    ts_deplint <command> -- <path1> <path2>

## Commands

    lint     Lint the passed-in paths for disallowed imports.
    diagram  Update README.md files in the passed-in paths with a Mermaid diagram of allowed imports.
    fix      Fix import violations in the passed-in paths by adding allow rules.
    format   Format the rules files in the passed-in paths.
