import matplotlib.pyplot as plt

room_width = 2000.0
room_height = 5000.0

stage_width = 1000.0
stage_height = 200.0

stage_bottom_left = [500.0, 0]

musicians = [0, 1, 0]
attendees = [[100,500], [200,1000], [1100,800]]

musician_positions = [[520, 20], [1000,100], [1450,20]]


border = room_width / 10

if room_width >= room_height:
    plt.figure("ICFPC 2023", figsize=(10, 10 * room_height / room_width))
else:    
    plt.figure("ICFPC 2023", figsize=(10 * room_width / room_height, 10))

plt.axis([-border, room_width + border, -border, room_height + border])

def plot_rect(bl, tr, col):
    x0 = bl[0]
    x1 = tr[0]
    y0 = bl[1]
    y1 = tr[1]
    plt.vlines(x = x0, ymin = y0, ymax = y1, colors = col)
    plt.vlines(x = x1, ymin = y0, ymax = y1, colors = col)
    plt.hlines(y = y0, xmin = x0, xmax = x1, colors = col)
    plt.hlines(y = y1, xmin = x0, xmax = x1, colors = col)
    
# room
plot_rect([0,0], [room_width,room_height], "black")

# stage
plot_rect(stage_bottom_left, [stage_bottom_left[0] + stage_width, stage_bottom_left[1] + stage_height], "blue")

# musicians
# TODO: color by instrument?
for p in musician_positions:
    plt.plot([p[0]], [p[1]], "ro")

for p in attendees:
    plt.plot([p[0]], [p[1]], "ro")

plt.show()
