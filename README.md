# zed-goose

Syntax highlighting for the [Goose](https://github.com/aardappel/goose)
programming language in [Zed](https://zed.dev), packaged as a self-contained
Zed extension. The grammar, its highlight queries and the snippets come from
the Goose project's own `tree-sitter/` and `vscode/` directories (Apache-2.0,
see LICENSE).

The extension carries its grammar in `grammar/` and points
`[grammars.goose]` in `extension.toml` at this very repository: Zed clones it
at the pinned `rev` and compiles `grammar/src/parser.c` + `scanner.c` to WASM
with [wasi-sdk](https://github.com/WebAssembly/wasi-sdk), which it downloads
automatically. The generated parser is therefore committed under
`grammar/src/`, unlike in the Goose repository where it is generated locally.

## Installing

Anyone with the repository cloned (or you, locally):

1. Install Rust (rustup); Zed adds the `wasm32-wasip2` target itself. The
   system Rust will not do unless it ships a wasm32-wasip2 standard library
   (Fedora's does not).
2. In Zed, run the command **install dev extension** and pick this directory.
3. Open a `.goose` file. The first install takes a minute while Zed compiles
   the extension crate and the grammar.

If install fails, `zed: open log` shows why; `zed --foreground` shows more.
Note: until this repository is pushed to GitHub, the clone in
`[grammars.goose].repository` cannot be checked out, so install the working
copy in `goose/b/zed-goose` instead (it uses a local `file://` grammar URL).

## Updating the grammar

`grammar/grammar.js` is a copy of the Goose repository's
`tree-sitter/grammar.js` (as are `scanner.c` and the queries). To pick up an
upstream change, or to change the grammar:

```sh
# sync from the Goose checkout, if that is where the change lives
cp ../goose/tree-sitter/grammar.js grammar/
cp ../goose/tree-sitter/src/scanner.c grammar/src/

cd grammar && python3 build.py && cd ..   # regenerate src/parser.c etc.
git add grammar && git commit -m "Sync grammar"
./update-grammar-rev                      # repin rev and commit
```

Then reinstall the dev extension (Zed rebuilds the Rust crate by itself when
files change, but not a new grammar `rev`).

## Publishing to the Zed extension registry

1. Push this repository to GitHub as `a55uka/zed-goose` (or edit the two
   `repository` URLs in `extension.toml` first if it lives elsewhere). The
   repo must be public, and published commits must sit on a branch.
2. Fork `zed-industries/extensions` with your personal account, clone it, and
   `git submodule update --init`.
3. Add the extension and an entry, then sort and push:

   ```sh
   git submodule add https://github.com/a55uka/zed-goose.git extensions/goose
   git add extensions/goose
   printf '[goose]\nsubmodule = "extensions/goose"\nversion = "0.1.0"\n' >> extensions.toml
   pnpm sort-extensions   # or sort the files by hand
   ```

   The `version` must equal `extension.toml`'s at the pinned commit.
4. Open a pull request: one extension per PR, at most three open at a time,
   and answer review within three weeks. After merge it is packaged into the
   registry, where users install it as "Goose".

For updates, bump `version` in `extension.toml`, commit, then PR the new
submodule pointer and `extensions.toml` version. The publishing guidelines
(https://zed.dev/docs/extensions/publishing/overview) have the details.
