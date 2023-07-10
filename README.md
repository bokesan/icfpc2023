# ICFP Contest 2023

Team: AStormOfMinds

Members:

- Christoph Breitkopf (Hannover, Germany)
- Jan Dreske (Vienna, Austria)

## Tools

We used Python for experimentation, exploration, and visualization,
and Rust for out actual solver.

## Solution approach

- Place musicians in regular rectangular grid on the whole stage.
- Optimize placement using simulated annealing. New placements are
  generated randomly using the following operations:
    - Swapping the positions of two musicians
    - Moving a musician
    - Changing the volume of one musician

## Problems encountered / Lessons learned

- Bugs in our (incremental) score computation consumed much time
- Some trivial tasks like the initial plament in a grid took way too much time
- Our optimization converges too slowly and required a lot of code tuning
  to even get a result. We should have investigated other approaches with faster
  convergance.
- We should have spent more time trying to solve problems by hand, or
  to improve computed solutions manually. That would have required writing
  an interactive visual solver tool, though, for which we lacked time.

## Overall

Fun problem and short and readble task specification.

- Suitable for both one-person and larger teams
- The websize and REST API were very helpful
- The contest was a lot of fun. Big thanks to the organizers!
