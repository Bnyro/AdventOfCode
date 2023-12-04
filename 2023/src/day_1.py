num_map = {
    "one": "o1e",
    "two": "t2o",
    "three": "t3e",
    "four": "f4r",
    "five": "f5e",
    "six": "s6x",
    "seven": "s7n",
    "eight": "e8t",
    "nine": "n9e"
}

def gen(input):
    lines = [''.join(filter(str.isdigit, line)) for line in input.split("\n")]
    lines = [line for line in lines if len(line) > 0]
    return sum(10 * int(nums[0]) + int(nums[-1]) for nums in lines)

with open("./inputs/day_1.txt", "r") as f:
    input_one = f.read()
    print("1_1: ", gen(input_one))

    input_two = input_one
    for key, value in num_map.items():
        input_two = input_two.replace(key, value)
    print("1_2: ", gen(input_two))
