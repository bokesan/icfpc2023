import math

# points are represented as a list [x,y]

def point_diff(a, b):
    return [a[0] - b[0], a[1] - b[1]]

def dot(a, b):
    return a[0] * b[0] + a[1] * b[1]

def dotself(a):
    x = a[0]
    y = a[1]
    return x*x + y*y

# line segment - circle intersection
# copies from: https://stackoverflow.com/questions/1073336/circle-line-segment-collision-detection-algorithm
def line_circle_intersect(E, L, C, r):
    d = point_diff(L, E)
    f = point_diff(E, C)
    a = dotself(d)
    b = 2 * dot(f, d)
    c = dotself(f) - r*r
    discriminant = b*b-4*a*c
    if discriminant < 0:
        return False
    else:
        # ray didn't totally miss sphere,
        # so there is a solution to
        # the equation.  
        discriminant = math.sqrt(discriminant)
        # either solution may be on or off the ray so need to test both
        # t1 is always the smaller value, because BOTH discriminant and
        # a are nonnegative.
        a2 = 2*a
        t1 = (-b - discriminant)/a2
        t2 = (-b + discriminant)/a2

        # 3x HIT cases:
        #          -o->             --|-->  |            |  --|->
        # Impale(t1 hit,t2 hit), Poke(t1 hit,t2>1), ExitWound(t1<0, t2 hit), 

        # 3x MISS cases:
        #       ->  o                     o ->              | -> |
        # FallShort (t1>1,t2>1), Past (t1<0,t2<0), CompletelyInside(t1<0, t2>1)
  
        if  t1 >= 0 and t1 <= 1:
            # t1 is the intersection, and it's closer than t2
            # (since t1 uses -b - discriminant)
            # Impale, Poke
            return True

        # here t1 didn't intersect so we are either started
        # inside the sphere or completely past it
        if  t2 >= 0 and t2 <= 1:
            # ExitWound
            return True
  
        # no intn: FallShort, Past, CompletelyInside
        return False
