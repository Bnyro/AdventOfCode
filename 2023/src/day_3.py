with open("./inputs/day_3.txt", "r") as f:
    content = f.read()
    lines = content.split("\n")
    lines_count = len(lines)

def is_schematic(line_index, line_length, start_index, end_index):
    left_index = max(start_index - 1, 0)
    right_index = min(end_index + 1, line_length - 1)
    top_index = max(line_index - 1, 0)
    bottom_index = min(lines_count - 1, line_index + 1)

    found = False
    star_positions = []

    for line_idx, line in enumerate(lines[top_index:bottom_index+1]):
        for char_idx, char in enumerate(line[left_index:right_index+1]):
            if not str.isdigit(char) and char != ".":
                found = True

            height = line_idx + top_index
            width = char_idx + left_index
            if char == "*" and not (height, width) in star_positions:
                star_positions.append((height, width))

    return found, star_positions
    

nums = []
num_positions = []
ratio_stars = []
gear_ratios = []
for line_index, line in enumerate(content.split("\n")):
    char_index = 0
    current_num_str = ""

    while char_index != len(line):
        if str.isdigit(line[char_index]):
            current_num_str += line[char_index]
        
        if current_num_str and (not str.isdigit(line[char_index]) or char_index == len(line) - 1):
            end_idx = char_index if str.isdigit(line[char_index]) else char_index - 1
            start_idx = end_idx - len(current_num_str) + 1

            schematic, star_positions = is_schematic(line_index, len(line), start_idx, end_idx)
            if schematic:
                nums.append(int(current_num_str))
                num_positions.append(
                    {
                        "num": int(current_num_str),
                        "star_pos": star_positions
                    }
                )
                
            current_num_str = ""
        char_index += 1

for pos1 in num_positions:
    for pos2 in num_positions:
        for star_pos1 in pos1["star_pos"]:
            for star_pos2 in pos2["star_pos"]:
                left = min(pos1["num"], pos2["num"])
                right = max(pos1["num"], pos2["num"])
                if star_pos1 == star_pos2 and left != right  and (star_pos1, left, right) not in ratio_stars:
                    ratio_stars.append((star_pos1, left, right))
                    gear_ratios.append(left * right)

print("3_1: ", sum(nums))
print("3_2: ", sum(gear_ratios))
