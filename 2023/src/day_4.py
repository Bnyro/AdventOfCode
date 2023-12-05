with open("./inputs/day_4.txt", "r") as f:
    content = f.read()

def get_card_won_count(card):
    winning_part = card.split(" | ")[0]
    provided_part = card.split(" | ")[1]
    winning_nums = [int(num) for num in winning_part.split(" ") if num]

    card_won_count = 0
    for num in provided_part.split(" "):
        if not num:
            continue
        
        if int(num) in winning_nums:
            card_won_count += 1

    return card_won_count

cards_to_handle = {}
card_values = []
card_count = 0

for card_index, line in enumerate(content.split("\n")[:-1]):
    card_won_count = get_card_won_count(line.split(": ")[1])

    weight = 1 + cards_to_handle.get(str(card_index), 0)
    card_count += weight
    
    if card_won_count != 0:
        value = pow(2, card_won_count - 1)
        card_values.append(value)

        for index in range(card_won_count):
            new_card_index = card_index + index + 1
            cards_to_handle[str(new_card_index)] = cards_to_handle.get(str(new_card_index), 0) + weight

print("4_1: ", sum(card_values))
print("4_2: ", card_count)
