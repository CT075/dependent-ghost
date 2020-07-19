## What is this?

`dependent-ghost` is a library that provides some of the benefits of dependent
typing to Rust via the [Ghosts of Departed Proofs](https://kataskeue.com/gdp.pdf)
technique. Library authors can provide APIs with statically-checked preconditions
and invariants, which the *user* can validate however necessary.

All credit for this technique goes to Matt Noonan, the author of the original
paper.

