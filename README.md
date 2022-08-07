# Rouge

Rouge (pronounced 'rooj', like the Louisiana state capital of Baton Rouge) is a statically-typed programming language designed for two primary uses: applications (graphical and command-line), and embedding into native programs (plugins, config files). To be suitable for both use cases, Rouge aims to have the following feature set:

 - A memory management model that aims to be intuitive and fast but with at least some guarantees towards memory and thread safety.
	- Current thoughts: reference counting with copy-on-write
 - A simple, easy-to-learn syntax inspired primarily by Ruby and Lua.
 - Interpreted for development and use in config files, bytecode-compiled for distribution.

The custom runtime environment (RTE) for the Rouge programming language will be provided as a Rust library (and someone can probably work on making a wrapping cdylib for interfacing with other languages) for embedding, and as a standalone utility for applications. Both will simply be called `rouge` and contain everything necessary to run and compile Rouge code.

Rouge is currently licensed under MIT.

For the project's Code of Conflict, please click [here](./CONFLICT.md).

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

 - [ ] Create a toolchain (e.g. dependency/project manager, doctool, language server)

### Low Priority

 - [ ] Branding.
