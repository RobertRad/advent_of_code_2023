#!/usr/bin/env python

import os
import re

sum = 0

def parse_numbers(number_string):
  numbers = number_string.strip().split(" ")
  numbers = list(filter(lambda num: num != '', numbers))
  return list(map(lambda num: int(num), numbers))

def parse_line(line):
  line = re.sub("Card +\d+: ", "", line)
  (winning_numbers, my_numbers) = line.split("|")
  winning_numbers = parse_numbers(winning_numbers)
  my_numbers = parse_numbers(my_numbers)
  return { "winning_numbers": winning_numbers, "my_numbers": my_numbers }

lines = ""
with open(f"{os.path.dirname(__file__)}/../input.txt") as f:
  lines = f.readlines()

cards = [parse_line(x) for x in lines]
for card in cards:
  # print(f"card: {card}")
  winning_numbers = set(card["winning_numbers"])
  num_contained = 0
  for my_number in card["my_numbers"]:
    if my_number in winning_numbers:
      num_contained += 1
  value = 0 if num_contained == 0 else 2 ** (num_contained - 1)
  sum += value
  # print(f"Contained: {num_contained}, {value}")

print(f"Part1: {sum}")
