# Project Euler Solutions

## Warning

**LINGUA DOES NOT INTEND TO LEAK PROJECT EULER SOLUTIONS.**

**THE SOLUTIONS HERE ARE KEPT AS AN EXTENDED TEST AND PROOF OF THE LINGUA LANGUAGE.**

If you want to solve Project Euler problems yourself, look away now and register on the [Project Euler website](https://projecteuler.net/).

## Layout

- `problemNNN.lf` solves problem `NNN`. Its examples run the small cases; `problemNNN.md` links the problem.
- `helpers.lf` holds what several problems share.
- `euler.lf` has `main`, which runs every problem at full size and asserts each answer.

Every `.lf` file sits directly in this directory because a lingua1 package is flat: an import names `{source_root}/<key>.lf`, and a file in a subdirectory is `L016`.

## Important Links

- [Project Euler](https://projecteuler.net/)
