from scipy.optimize import linprog
import numpy as np

with open("data.txt") as file:

	x_1 = 0
	x_2 = 0
	y_1 = 0
	y_2 = 0
	x_target = 0
	y_target = 0
	c = [1, 0]
	cost = 0
	for (index, row) in enumerate(file):
		row = row.strip()
		if index % 4 == 0:
			part1, part2 = row.split(",")
			x_1 = int(part1.split("+")[1])
			y_1 = int(part2.split("+")[1])
		if index % 4 == 1:
			part1, part2 = row.split(",")
			x_2 = int(part1.split("+")[1])
			y_2 = int(part2.split("+")[1])
		if index % 4 == 2:
			part1, part2 = row.split(",")
			x_target = int(part1.split("=")[1])
			y_target = int(part2.split("=")[1])
		if index % 4 == 3:
			matrix = [[x_1, x_2], [y_1, y_2]]
			target = [x_target, y_target]
			integrality = np.ones_like(c)
			solution = linprog(c, A_eq=matrix, b_eq=target, integrality=integrality)
			if solution.x is not None:
				cost += solution.x[0] * 3 + solution.x[1]

print(cost)