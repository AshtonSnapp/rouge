# Rouge

Rouge (pronounced 'rooj' like in French or 'rowg' like in English) is a statically-typed programming language designed for two primary uses: applications (graphical and command-line), and embedding into native programs (plugins, config files). To be suitable for both use cases, Rouge aims to have the following feature set:

 - A memory management model that aims to be intuitive but with at least some guarantees towards memory and thread safety.
	- Taking inspiration from Rust primarily in this regard.
 - A simple, easy-to-learn syntax inspired primarily by Ruby and Lua.
 - Interpreted for development and use in config files, bytecode-compiled for distribution.

While not entirely important to it's stated use cases, Rouge also aims to have tagged unions for enums, traits for shared functionality (rather than inheritance, which means Rouge may or may not be object-oriented depending on your definition), and probably some other stuff that I can't think of right now.

The custom runtime environment (RTE) for the Rouge programming language will be provided as a Rust library (and someone can probably work on making a wrapping cdylib for interfacing with other languages) for embedding, and as a standalone utility for applications. Both will simply be called `rouge` and contain everything necessary to run and compile Rouge code.

Rouge is currently licensed under the GNU LGPL v3 (or later), a lesser copyleft license ensuring the language itself remains open-source while allowing proprietary programs to be written using it (either embedding the runtime or being written in the language). If you think this is a bad idea, please explain why and provide a suggestion for a replacement.

## Current TODO List

### High Priority

 - [ ] Create a functioning RTE and compiler.
	- [ ] Design the RTE's instruction set and the bytecode file format.
	- [ ] Design an intuitive interface for communication between a native program and an embedded Rouge runtime.
	- [ ] Figure out how to implement JIT compilation.
 - [ ] Create the standard library.
 - [ ] Create documentation.
	- [ ] (Optional) Specifications (at least semi-formal) for the language and related things.
 - [ ] Create a functioning command-line utility.

### Medium Priority

 - [ ] Create a toolchain (e.g. dependency/project manager, doctool)

### Low Priority

 - [ ] Branding.