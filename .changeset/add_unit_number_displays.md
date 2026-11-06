---
default: major
---

#### Add `unit(...)` number displays

`display(unit("SOL"))`, or `display = unit(...)` inline on a number type, labels a number with a unit without scaling it, and the unit can also be a public key such as a mint address. Floats and fixed points take only this display, so an `amount(...)` display on them is now an error pointing to `unit(...)`, and an `amount(...)` display without `decimals` suggests either.
