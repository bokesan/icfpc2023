import sys
import math
import random
import time
from geometry import line_circle_intersect

# distribute musician positions in rows
def make_positions(problem, rows):
    stw = problem['stage_width']
    sth = problem['stage_height']
    n = len(problem['musicians'])
    if stw >= sth:
        max_rows = sth // 10 - 1
        max_per_row = stw // 10 - 1
    else:
        max_rows = stw // 10 - 1
        max_per_row = sth // 10 - 1
    if rows > max_rows:
        print(f"Stage too small: Reducing rows from {rows} to {max_rows}.")
        rows = max_rows
    n_per_row = (n + rows - 1) // rows
    while n_per_row > max_per_row:
        if rows >= max_rows:
            print(f"Stage too small - bailing out")
            return []
        rows = rows + 1
        n_per_row = (n + rows - 1) // rows
    print(f"Placing {n} musicians in {rows} rows: {n_per_row} per row.")
    if n_per_row * rows > n:
        print(f"  (Last row only {n - n_per_row * (rows - 1)})")
    positions = []
    if stw > sth:
        x_step = stw // (n_per_row + 1)
        y_step = sth // (rows + 1)
        y = y_step
        for r in range(rows):
            x = x_step
            for c in range(n_per_row):
                positions.append({'x': x, 'y': y})
                x = x + x_step
            y = y + y_step
    else:
        x_step = stw // (rows + 1)
        y_step = sth // (n_per_row + 1)
        x = x_step
        for c in range(rows):
            y = y_step
            for r in range(n_per_row):
                positions.append({'x': x, 'y': y})
                y = y + y_step
            x = x + x_step
    xoffs = problem['stage_bottom_left'][0]
    yoffs = problem['stage_bottom_left'][1]
    return list(map(lambda p: {'x': p['x'] + xoffs, 'y': p['y'] + yoffs}, positions[0:n]))

def is_blocked(placements, musician_index, attendee):
    attendee_pos = [attendee['x'], attendee['y']]
    musician_pos = [placements[musician_index]['x'], placements[musician_index]['y']]
    for i in range(len(placements)):
        p = placements[i]
        if i != musician_index and line_circle_intersect(attendee_pos, musician_pos, [p['x'], p['y']], 5):
            return True
    return False

def annotate_with_los(problem, positions):
    for i in range(len(positions)):
        visible = []
        for a in problem['attendees']:
            visible.append(not is_blocked(positions, i, a))
        positions[i]['visible'] = visible


def happiness1(a,m,instrument):
    # d = math.dist([a['x'],a['y']], [m['x'],m['y']])
    dx = a['x'] - m['x']
    dy = a['y'] - m['y']
    # not the exact official score, but faster without sqrt + squaring
    d2 = dx*dx + dy*dy
    return math.ceil(1000000.0 * a['tastes'][instrument] / d2)

def happiness(attendee_index, problem, placements):
    sum = 0
    ms = problem['musicians']
    if len(placements) != len(ms):
        print("Fatal error: wrong placements length")
        sys.exit(1)
    a = problem['attendees'][attendee_index]
    for k in range(len(ms)):
        place = placements[k]
        if place['visible'][attendee_index]:
            instrument = ms[k]
            sum = sum + happiness1(a, place, instrument)
    return sum

def score(problem, placements):
    sum = 0
    for ai in range(len(problem['attendees'])):
        sum = sum + happiness(ai, problem, placements)
    return sum

max_time = 120

def solve_fixed(problem):
    start_time = time.time()
    r = make_positions(problem, 2)
    print("Precomputing line-of-sound...")
    annotate_with_los(problem, r)
    s = score(problem, r)
    print(f"Initial score: {s}")
    perms = 1
    while (time.time() - start_time) < max_time:
        perms = perms + 1
        r2 = random.sample(r, len(r))
        s2 = score(problem, r2)
        if s2 > s:
            print(f"Score improved: {s2}")
            r = r2
            s = s2
    print(f"Permutations tested: {perms}")
    # remove annotation
    for p in r:
        del p['visible']
    return r
