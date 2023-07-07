import json

problem_id = 16
with open(f'problems/problem-{problem_id}.json', "r") as file:
    problem = json.loads(json.load(file))
print(f"Problem number {problem_id}")
print(f"room: {problem['room_width']} * {problem['room_height']}")
print(f"stage: {problem['stage_width']} * {problem['stage_height']} "
      f"at {problem['stage_bottom_left'][0]}/{problem['stage_bottom_left'][1]}")
print(f"musicians: {len(problem['musicians'])} with {len(set(problem['musicians']))} different instruments")
print(f"attendees: {len(problem['attendees'])}")
likes = {}
dislikes = {}
for a in problem['attendees']:
    for i in range(len(a['tastes'])):
        if i not in likes:
            likes[i] = 0
            dislikes[i] = 0
        taste = a['tastes'][i]
        if taste > 0:
            likes[i] = likes[i] + taste
        if taste < 0:
            dislikes[i] = dislikes[i] + taste
for i in range(len(set(problem['musicians']))):
    print(f"instrument {i}: likes {likes[i]}, dislikes {dislikes[i]}")
