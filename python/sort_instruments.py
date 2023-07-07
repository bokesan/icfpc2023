import score
import json
from collections import Counter


def sort_rows(problem):
    # todo do this for all permutations of instruments
    solutions = []
    size = len(set(problem['musicians']))
    for o in range(size):
        placements = []
        filled = {}
        x_base = problem['stage_bottom_left'][0] + 10
        y_base = problem['stage_bottom_left'][1] + 10
        for i in range(len(problem['musicians'])):
            instrument = (problem['musicians'][i] + o) % size  # rotate through instruments
            y = y_base + (instrument * 10)  # find the correct row to be in based on instrument
            if y not in filled:
                filled[y] = 0
            x = x_base + filled[y] * 10
            filled[y] = filled[y] + 1
            placements.append({"x": x, "y": y})
        solutions.append(placements)
    return solutions


def sort_columns(problem):
    # todo do this for all permutations of instruments
    solutions = []
    size = len(set(problem['musicians']))
    for o in range(size):
        placements = []
        filled = {}
        x_base = problem['stage_bottom_left'][0] + 10
        y_base = problem['stage_bottom_left'][1] + 10
        for i in range(len(problem['musicians'])):
            instrument = (problem['musicians'][i] + o) % size  # rotate through instruments
            x = x_base + (instrument * 10)  # find the correct column to be in based on instrument
            if x not in filled:
                filled[x] = 0
            y = y_base + filled[x] * 10
            filled[x] = filled[x] + 1
            placements.append({"x": x, "y": y})
        solutions.append(placements)
    return solutions


def solve(problem):
    solutions = []
    num_instruments = len(set(problem['musicians']))
    most_common, max_instruments = Counter(problem['musicians']).most_common(1)[0]
    row_size = (problem['stage_width'] - 10) // 10
    column_size = (problem['stage_height'] - 10) // 10
    if (row_size >= max_instruments) and (column_size >= num_instruments):
        solutions.extend(sort_rows(problem))
    if (row_size >= num_instruments) and (column_size >= max_instruments):
        solutions.extend(sort_columns(problem))
    best = None
    max_points = float('-inf')
    for sol in solutions:
        points = score.score(problem, sol)
        print(f"Found a solution with {points} points")
        if points > max_points:
            max_points = points
            best = sol
    print(f"Best solution found has {max_points} points")
    return best


def main():
    problem_id = 33
    with open(f'problems/problem-{problem_id}.json', "r") as file:
        problem = json.loads(json.load(file))
    print(f"Problem number {problem_id}")
    print(f"room: {problem['room_width']} * {problem['room_height']}")
    print(f"stage: {problem['stage_width']} * {problem['stage_height']} "
          f"at {problem['stage_bottom_left'][0]}/{problem['stage_bottom_left'][1]}")
    print(f"musicians: {len(problem['musicians'])} with {len(set(problem['musicians']))} different instruments")
    print(f"attendees: {len(problem['attendees'])}")
    placements = solve(problem)
    solution = {"placements": placements}
    with open(f'solutions/solution_{problem_id}.json', "w") as file:
        file.write(json.dumps(solution))
    print(f"Stored solution for problem #{problem_id}")


if __name__ == "__main__":
    main()
