time, distance = """
Time:        40     92     97     90
Distance:   215   1064   1505   1100
""".strip().split('\n')
times = list(map(int, time.split()[1:]))
distances_to_beat = list(map(int, distance.split()[1:]))

product = 1
for race_time, record in zip(times, distances_to_beat):
    count = 0
    for i in range(race_time + 1):
        travelled_distance = (race_time - i) * i
        count += travelled_distance > record
    product *= count

print(product)

# Part 2 merge to numbers
race_time = int(''.join(time.split()[1:]))
record = int("".join(distance.split()[1:]))
count = 0
for i in range(race_time + 1):
    travelled_distance = (race_time - i) * i
    count += travelled_distance > record

print(count)
