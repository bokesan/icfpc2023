#!/usr/bin/env python3

import sys
import re
import json
import math

from geometry import line_circle_intersect, distance

# Extension 2 enabled?
def playing_together(problem):
    return problem['id'] >= 56

def get_closeness_factors(problem, placements):
    ms = problem['musicians']
    m = len(ms)
    if not playing_together(problem):
        return [1] * m
    else:
        cls = []
        for i in range(m):
            p = placements[i]
            instrument = ms[i]
            sum = 0
            for j in range(m):
                if j != i and ms[j] == instrument:
                    sum += 1 / distance(p, placements[j])
            cls.append(sum + 1)
        return cls

def load_problem(path):
    with open(path, 'r') as f:
        problem_string = json.load(f)
        return json.loads(problem_string)

def load_solution(path):
    with open(path, 'r') as f:
        solution = json.load(f)
        return solution['placements']

def is_blocked(problem, placements, musician_index, attendee):
    attendee_pos = [attendee['x'], attendee['y']]
    musician_pos = [placements[musician_index]['x'], placements[musician_index]['y']]
    for i in range(len(placements)):
        p = placements[i]
        if i != musician_index and line_circle_intersect(attendee_pos, musician_pos, [p['x'], p['y']], 5):
            return True
    for p in problem['pillars']:
        c = p['center']
        c = [c[0], c[1]]
        if line_circle_intersect(attendee_pos, musician_pos, c, p['radius']):
            return True
    return False
            

def impact(a, m, instrument):
    d = distance(a,m)
    return math.ceil(1000000.0 * a['tastes'][instrument] / (d*d))

def happiness(a, problem, placements, closeness):
    sum = 0
    ms = problem['musicians']
    for k in range(len(ms)):
        if not is_blocked(problem, placements, k, a):
            place = placements[k]
            instrument = ms[k]
            sum += math.ceil(closeness[k] * impact(a, place, instrument))
    return sum

def calc_score(problem, placements):
    m = len(problem['musicians'])
    print(f"Problem {problem['id']}: musicians={m}, attendees={len(problem['attendees'])}, pillars={len(problem['pillars'])}")
    if len(placements) != len(problem['musicians']):
        print("Placements length does not match number of musicians")
        return 0
    closeness = get_closeness_factors(problem, placements)
    print(f"Closeness: {closeness}")
    sum = 0
    for a in problem['attendees']:
        sum += happiness(a, problem, placements, closeness)
    return sum

def process(pf,sf):
    match = re.search(r"problem-([0-9]+).json", pf)
    if match:
        id = int(match.group(1))
        if id <= 0 or id > 1000000:
            print(f"Invalid problem id: {id}")
            sys.exit(1)
        problem = load_problem(pf)
        # add the id to the problem
        problem['id'] = id
        placements = load_solution(sf)
        score = calc_score(problem, placements)
        print(f"Score: {score}")

process(sys.argv[1], sys.argv[2])
