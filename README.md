# nc

Modern replacement to nc.

# Additional features

`-R` to replace. Helpful when you want additional formatting for each packet coming in.

```bash
nc -luR '$0\n' 8125
```

Listen for UDP packets on 8125, printing each packet on a newline.

# Installation

1. Clone
2. `just install` or `cargo install --path .`