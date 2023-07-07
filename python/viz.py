import math
import matplotlib.pyplot as plt
import sys
import os
import json

# max. window size in inches?
win_size = 10

problem = {}

def load_problem(path):
    global problem
    with open(path, 'r') as f:
        problem_string = json.load(f)
        problem = json.loads(problem_string)

if len(sys.argv) < 2:
    print("Usage: python viz.py <problem-file>")
    sys.exit(1)
        
load_problem(sys.argv[1])

def is_blocked(placements, musician_index, attendee):
    # TODO
    return False

def happiness1(a,m,instrument):
    # d = math.dist([a['x'],a['y']], [m['x'],m['y']])
    dx = a['x'] - m['x']
    dy = a['y'] - m['y']
    d2 = dx*dx + dy*dy
    return math.ceil(1000000.0 * a['tastes'][instrument] / d2)

def happiness(a,problem,placements):
    sum = 0
    ms = problem['musicians']
    for k in range(len(ms)):
        if not is_blocked(placements, i, a):
            place = placements[i]
            instrument = ms[i]
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


num_musicians = len(problem['musicians'])
placements = []
pos = {'x': problem['stage_bottom_left'][0] + 20, 'y': problem['stage_bottom_left'][1] + 12}
print(f'Placing all musicians at {pos} (illegal, I know)')
for i in range(num_musicians):
    placements.append(pos)
    
# musicians
for p in placements:
    plt.plot([p['x']], [p['y']], "go")

plt.show()
    
print(f'Score (blocking not implemented): {score(problem, placements)}')

solution = {'placements' : placements}
with open('placements.json', 'w') as fp:
    json.dump(solution, fp)
print("placements written to 'placements.json'")

