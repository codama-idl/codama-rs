---
default: major
---

#### Update program, account and instruction nodes for spec v2

Instructions describe their payload with a single optional `data` type node, plus an optional `return_data`, instead of a list of arguments. Accounts accept any type node as their data, remaining accounts are identified by name rather than carrying an argument or resolver value, and byte deltas use the spec v2 value nodes. Programs no longer have an `origin`, error and status messages accept rich text, and plugin namespaces are validated, so invalid ones are rejected when deserialising.
