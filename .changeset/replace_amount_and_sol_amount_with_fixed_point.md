---
default: major
---

#### Replace `amount` and `sol_amount` with `fixed_point`, and add `duration` and ticks

`amount(number(u64), 6, "USDC")` becomes `fixed_point(integer(u64), scale = 6, unit = "USDC")`, with an optional `base`, and `sol_amount(...)` becomes `fixed_point(..., scale = 9, unit = "SOL")`. `integer(...)` and `float(...)` accept a `unit`, `date_time(...)` accepts `ticks_per_second`, and the new `duration(...)` type describes time spans. Using `amount(...)` or `sol_amount(...)` fails with a hint pointing to the replacement.
