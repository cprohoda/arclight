# arclight

-- Currently working on this to learn rust --
-- Very incomplete --

Arclight is going to be a lisp-like language which compiles to rust.
It aims for (1) safety, (2) performance, (3) maintainability, and (4) rapid prototyping.

Language philosophy (and the relevant goals):
-All assumed data should be made explicit in the source code (1,3,4)
-All implementation information should be default-hidden but easily accessed (3,4)
-Analytical and sample profiler should be a core language/environment feature (2)
-No undefined or implementation-defined behavior (1,3)
-Strict scoping can defined by the structure of the source code (1,3,4)
-Meta-information should be used to enhance development (3,4)
---Code & version info bundling as an alternative approach to deprecation (3)
-----Version info hard-coded but changeable by version update program
---Type/test information can frequently be automatically generated during development (3,4)
-----Use debug information with debug runs, type-generated sample data, and programmer reviews to develop tests
-Systems language with sensible defaults and wrappers the best combination for (2,4)
---Lisp-like language on top of rust
-Language must be both interpreted and compiled, but the language rules need not be the same for both (3,4)
---All compileable programs must be interpretable, but not all interpretable programs must be compilable (i.e. compilation is stricter & safer)
---Interpreter as non-default module of compiler
---Interpreter must be a fully-available tool running on top of ide for development ease

Planned features:
-

Todo:
-Write language
-Write ide
-Look into dependent type systems