import sys
import re
import json
import math

from geometry import line_circle_intersect

def load_problem(path):
    with open(path, 'r') as f:
        problem_string = json.load(f)
        return json.loads(problem_string)

def load_solution(path):
    with open(path, 'r') as f:
        solution = json.load(f)
        return solution['placements']
def is_blocked(placements, musician_index, attendee):
    attendee_pos = [attendee['x'], attendee['y']]
    musician_pos = [placements[musician_index]['x'], placements[musician_index]['y']]
    for i in range(len(placements)):
        p = placements[i]
        if i != musician_index and line_circle_intersect(attendee_pos, musician_pos, [p['x'], p['y']], 5):
            return True
    return False
            

def happiness1(a,m,instrument):
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

def calc_score(problem, placements):
    if len(placements) != len(problem['musicians']):
        print("Placements length does not match number of musicians")
        return 0
    sum = 0
    for a in problem['attendees']:
        sum = sum + happiness(a, problem, placements)
    return sum

def process(pf,sf):
    match = re.search(r"problem-([0-9]+).json", pf)
    if match:
        id = int(match.group(1))
        if id <= 0 or id > 1000000:
            print(f"Invalid problem id: {id}")
            sys.exit(1)
        problem = load_problem(pf)
        placements = load_solution(sf)
        score = calc_score(problem, placements)
        print(f"Score: {score}")

process(sys.argv[1], sys.argv[2])
