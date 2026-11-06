---
default: major
---

#### Split `number` into `integer` and `float`, and replace `argument` with `field` and `data`

Write `integer(...)` or `float(...)` wherever you used `number(...)`, including in `size = ...` and `prefix = ...`. Instruction arguments are declared as data fields with `field(...)`, values that read from instruction data use `data(...)`, which also accepts nested paths such as `data("config.fees[0]")`, and `remaining_accounts(argument("x"), ...)` becomes `remaining_accounts("x", ...)`. Amount displays now require `decimals`, and enum variants accept any payload type. Using a removed spelling fails with a hint pointing to its replacement.
