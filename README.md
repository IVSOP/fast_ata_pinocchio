# What

A utility crate for using ATAs, similar to spl associated-token-account

This is an anchor port inspired by https://github.com/anza-xyz/pinocchio/blob/main/programs/associated-token-account

# Why

The SPL version has a lot of things I don't like:

- Bloat
- Inneficient, don't allow reusing bumps
- Implicit things such as program IDs
- Magic numbers instead of just using enums
- Have the user guess what accounts are needed to call the instruction. Might as well invoke it ourselves

# TODO

- Testing (lite SVM?)
- Other instructions, for now I only have Create
- Examples
