import json


def solve(id):
    with open(f'problems/problem-{id}.json', "r") as file:
        problem = json.loads(json.load(file))

    print(f"Problem number {id}")
    print(f"room: {problem['room_width']} * {problem['room_height']}")
    print(f"stage: {problem['stage_width']} * {problem['stage_height']} "
          f"at {problem['stage_bottom_left'][0]}/{problem['stage_bottom_left'][1]}")
    print(f"musicians: {len(problem['musicians'])} with {len(set(problem['musicians']))} different instruments")
    print(f"attendees: {len(problem['attendees'])}")

    placements = []
    x = problem['stage_bottom_left'][0] + 10
    y = problem['stage_bottom_left'][1] + 10
    for i in range(len(problem['musicians'])):
        if y > problem['stage_height'] + problem['stage_bottom_left'][1] - 10:
            print("not enough space")
            return
        placements.append({"x": x, "y": y})
        x = x + 10
        if x > problem['stage_width'] + problem['stage_bottom_left'][0] - 10:
            x = problem['stage_bottom_left'][0] + 10
            y = y + 10

    solution = {"placements": placements}
    with open(f'solutions/solution_{id}.json', "w") as file:
        file.write(json.dumps(solution))
    print(f"{len(placements)} musicians placed")
    print()


for p in range(45):
    solve(p+1)



