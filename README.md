# ICFP Contest 2023

Team: AStormOfMinds

Members:

- Christoph Breitkopf (Hannover, Germany)
- Jan Dreske (Vienna, Austria)

## Tools

We used Python for experimentation, exploration, and visualization,
and Rust for our actual solver.

## Solution approach

- Place musicians in a regular rectangular grid on the whole stage.
- Optimize placement using simulated annealing. New placements are
  generated randomly using the following operations:
    - Swapping the positions of two musicians
    - Moving a musician
    - Changing the volume of one musician
 
We did not do well this year, scoring only about half of the top 10 scores,
and probably ended up somewhere from place 65-80.

## Problems encountered / Lessons learned

- Bugs in our (incremental) score computation consumed much time
- Some trivial tasks, like the initial placement in a grid, took way too much time
- Our optimization converges too slowly and requires much code tuning to get a result.
  We should have investigated other approaches with faster
  convergence.
- We would have needed incremental score computation and a faster line-of-sound algorithm for better
  performance. We looked at a scan-line approach for
  the latter but failed to see how to adapt that to the problem.
- We should have spent more time solving problems by hand or
  improving computed solutions manually. That would have required writing
  an interactive visual solver tool, though, for which we lacked time.
- A live visualization during the optimization would likely have been very
  helpful to see if the optimization converges and give hints for parameter
  tuning.

## Overall

Fun problem and short and readable task specification.

- Suitable for both one-person and larger teams
- The website and REST API were very helpful
- The contest was a lot of fun. Big thanks to the organizers!
