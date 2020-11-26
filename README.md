<p align="center">
	<img src="./docs/logo.svg" width="450">
  <br />
	<img src="https://github.com/safinsingh/delta/workflows/CI/badge.svg" />
	<img src="https://img.shields.io/badge/Made%20With-Rust-orange?logo=rust" />
</p>

<p align="center">
	<i>
		A simple yet powerful interpreted programming language
	</i>
</p>

<hr />

- [x] lexer
  - [x] ops
  - [x] strings
  - [x] idents/keywords
  - [x] comments
  - [x] delimeters (both `\n` and `;`)
- [ ] parser
  - [x] convert token stream to postfix ops
  - [ ] parse tree generator
    - [x] binary exprs
    - [x] unary exprs
    - [x] literals (i think)
    - [ ] fn calls, fn defs
    - [ ] match patterns
  - [ ] `[LONG-TERM]` type-check tree
  - [ ] `[LONG-TERM]` bytecode generator
- [ ] evaluator
  - [ ] impl std::ops::\* for NodeResult
    - [x] add
    - [x] sub
    - [x] (logical) not
- [ ] vm
  - [ ] error handling
