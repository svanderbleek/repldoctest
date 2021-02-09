# repldoctest

install

```
cargo install --git https://github.com/svanderbleek/repldoctest.git
```

take a function name and a repl interaction of examples

```
$ repldoctest minCost '*Assignment05> minCost (TheInt 3) (TheInt 4)
TheInt 3
*Assignment05> minCost (TheInt 3) Inf
TheInt 3
*Assignment05> minCost Inf (TheInt 4)
TheInt 4
*Assignment05> minCost Inf Inf
Inf'
```

output a haskell doctest for that function using the examples

```
-- | minCost
-- >>> minCost (TheInt 3) (TheInt 4)
-- TheInt 3
-- >>> minCost (TheInt 3) Inf
-- TheInt 3
-- >>> minCost Inf (TheInt 4)
-- TheInt 4
-- >>> minCost Inf Inf
-- Inf
```

## todo

- [ ] change hardcoded replace to regex
- [ ] handle linewrap
