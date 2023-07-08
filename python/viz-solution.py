import math
import matplotlib.pyplot as plt
import sys
import os
import json

from geometry import line_circle_intersect
from fixed_place_solver import solve_fixed

# max. window size in inches?
win_size = 10

problem = {}

def load_problem(path):
    global problem
    with open(path, 'r') as f:
        problem_string = json.load(f)
        problem = json.loads(problem_string)

def load_solution(path):
    with open(path, 'r') as f:
        solution = json.load(f)
        return solution['placements']
        
if len(sys.argv) < 2:
    print("Usage: python viz.py [-batch] <problem-file>")
    sys.exit(1)

load_problem(sys.argv[1])

def is_blocked(placements, musician_index, attendee):
    attendee_pos = [attendee['x'], attendee['y']]
    musician_pos = [placements[musician_index]['x'], placements[musician_index]['y']]
    for i in range(len(placements)):
        p = placements[i]
        if i != musician_index and line_circle_intersect(attendee_pos, musician_pos, [p['x'], p['y']], 5):
            return True
    return False
            

def happiness1(a,m,instrument):
    # d = math.dist([a['x'],a['y']], [m['x'],m['y']])
    dx = a['x'] - m['x']
    dy = a['y'] - m['y']
    d = math.sqrt(dx*dx + dy*dy)
    return math.ceil(1000000.0 * a['tastes'][instrument] / (d*d))

def happiness(a,problem,placements):
    sum = 0
    ms = problem['musicians']
    for k in range(len(ms)):
        if not is_blocked(placements, k, a):
            place = placements[k]
            instrument = ms[k]
            sum = sum + happiness1(a, place, instrument)
    return sum

def score(problem, placements):
    sum = 0
    for a in problem['attendees']:
        sum = sum + happiness(a, problem, placements)
    return sum
            
def plot_rect(bl, tr, col):
    x0 = bl[0]
    x1 = tr[0]
    y0 = bl[1]
    y1 = tr[1]
    plt.vlines(x = x0, ymin = y0, ymax = y1, colors = col)
    plt.vlines(x = x1, ymin = y0, ymax = y1, colors = col)
    plt.hlines(y = y0, xmin = x0, xmax = x1, colors = col)
    plt.hlines(y = y1, xmin = x0, xmax = x1, colors = col)

def plot_problem(problem):
    rw = problem['room_width']
    rh = problem['room_height']
    if rw >= rh:
        plt.figure("ICFPC 2023", figsize=(win_size, win_size * rh / rw))
        border = rw / 10
    else:    
        plt.figure("ICFPC 2023", figsize=(win_size * rw / rh, win_size))
        border = rh / 10
    plt.axis([-border, rw + border, -border, rh + border])
    # room
    plot_rect([0,0], [rw,rh], "black")
    sw = problem['stage_width']
    sh = problem['stage_height']
    sbl = problem['stage_bottom_left']
    # stage
    plot_rect(sbl, [sbl[0] + sw, sbl[1] + sh], "blue")

    for a in problem['attendees']:
        plt.plot([a['x']], [a['y']], "ro")

plot_problem(problem)


# copied from simple solver
def simple_solver(problem):
    placements = []
    x = problem['stage_bottom_left'][0] + 10
    y = problem['stage_bottom_left'][1] + 10
    for i in range(len(problem['musicians'])):
        if y > problem['stage_height'] + problem['stage_bottom_left'][1] - 10:
            print("not enough space")
            return []
        placements.append({"x": x, "y": y})
        x = x + 10
        if x > problem['stage_width'] + problem['stage_bottom_left'][0] - 10:
            x = problem['stage_bottom_left'][0] + 10
            y = y + 10
    return placements

def on_stage(x, y):
    bl = problem['stage_bottom_left']
    blx = bl[0]
    bly = bl[1]
    h = problem['stage_height']
    w = problem['stage_width']
    return x >= blx + 10 and y >= bly + 10 and x <= blx + w - 10 and y <= bly + h - 10

placements = load_solution(sys.argv[2])
if len(placements) != len(problem['musicians']):
    print("Bad solution: wrong number of placements")
    sys.exit(1)

# musicians
print(f"Stage: {problem['stage_bottom_left'][0]},{problem['stage_bottom_left'][1]} - {problem['stage_bottom_left'][0]+problem['stage_width']},{problem['stage_bottom_left'][1]+problem['stage_height']}")
for p in placements:
    if not on_stage(p['x'], p['y']):
        print(f"Musician not on stage: {p}")
    plt.plot([p['x']], [p['y']], "go")
plt.show()

print(f"Computing score for {len(problem['attendees'])} attendees ...")
s = score(problem, placements)
print(f'Score: {s}')

