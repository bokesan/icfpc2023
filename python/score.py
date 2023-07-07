import math
from geometry import line_circle_intersect


def is_blocked(placements, musician_index, attendee):
    attendee_pos = [attendee['x'], attendee['y']]
    musician_pos = [placements[musician_index]['x'], placements[musician_index]['y']]
    for i in range(len(placements)):
        p = placements[i]
        if i != musician_index and line_circle_intersect(attendee_pos, musician_pos, [p['x'], p['y']], 5):
            return True
    return False


def happiness1(a, m, instrument):
    dx = a['x'] - m['x']
    dy = a['y'] - m['y']
    d = math.sqrt(dx * dx + dy * dy)
    return math.ceil(1000000.0 * a['tastes'][instrument] / (d * d))


def happiness(a, problem, placements):
    point_sum = 0
    ms = problem['musicians']
    for k in range(len(ms)):
        if not is_blocked(placements, k, a):
            place = placements[k]
            instrument = ms[k]
            point_sum = point_sum + happiness1(a, place, instrument)
    return point_sum


def score(problem, placements):
    point_sum = 0
    for a in problem['attendees']:
        point_sum = point_sum + happiness(a, problem, placements)
    return point_sum
