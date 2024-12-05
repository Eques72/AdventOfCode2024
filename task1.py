# Maybe the lists are only off by a small amount! To find out, pair up 
# the numbers and measure how far apart they are. Pair up the smallest number 
# in the left list with the smallest number in the right list, then the 
# second-smallest left number with the second-smallest right number, and so on.

# Within each pair, figure out how far apart the two numbers are; you'll 
# need to add up all of those distances. For example, if you pair up a 3 
# from the left list with a 7 from the right list, the distance apart is 4; 
# if you pair up a 9 with a 3, the distance apart is 6.


import csv
import time

#Input
list_1 = []
list_2 = []
part = 2

# load lists
with open('input_task_1.csv', 'r', newline='') as file:
    reader = csv.reader(file, delimiter=' ')
    for row in reader:
        list_1.append(int(row[0]))
        list_2.append(int(row[1]))

#Part 1
if part == 1:
    start = time.time()

    list_1 = sorted(list_1) #asc
    list_2 = sorted(list_2)

    end = time.time()
    print("Time of sorting 2 lists: ", end - start)

    distane_sum = 0
    for i in range(len(list_1)):
        distane_sum += abs(list_1[i] - list_2[i])

    print("Sum of distances: ", distane_sum)

#Part 2
if part == 2:
    similarity_score = 0
    matches = 0
    map_2 = {}
    for loc in list_2:
        if loc not in map_2:
            map_2[loc] = 1
        else:
            map_2[loc] += 1

    for loc in list_1:
        if loc in map_2:
            similarity_score += map_2[loc] * loc
            matches += 1
            print("loc: ", loc, " map_2[loc]: ", map_2[loc], " matches: ", matches)
    print("Similarity score: ", similarity_score)