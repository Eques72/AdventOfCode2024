from itertools import product
import operator

def read_data(file_path):
    sums = []
    ingredients = []

    with open(file_path, 'r') as file:
        for line in file:
            parts = line.strip().split(':')
            if len(parts) == 2:
                sums.append(int(parts[0].strip()))   
                ingredients.append([int(num) for num in parts[1].strip().split()])

    return sums, ingredients

def concatenation_operator(a, b):
    return int(f"{a}{b}")

file_path = 'input_task_7.txt'
sums, ingredients = read_data(file_path)

results = set()
for i in range(len(sums)):
    operators = [operator.add, operator.mul, concatenation_operator]
    combinations_operators = product(operators, repeat=len(ingredients[i]) - 1)
    for comb in combinations_operators:
        result = ingredients[i][0]
        for j, op in enumerate(comb):
            result = op(result, ingredients[i][j + 1])
        if result == sums[i]:
            results.add(sums[i])

print(sum(results))