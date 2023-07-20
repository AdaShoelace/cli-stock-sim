#!/bin/python3

import random
import string

def generate_variations(company_name, num_variations, hamming_distance):
    variations = []
    for _ in range(num_variations):
        variation = list(company_name)
        indices_to_change = random.sample(range(len(company_name)), hamming_distance)
        for idx in indices_to_change:
            variation[idx] = random.choice(string.ascii_lowercase)
        variations.append(''.join(variation))
    return variations

# Example usage
company_name = 'Mercedes'
num_variations = 1
hamming_distance = 1

variations = generate_variations(company_name, num_variations, hamming_distance)
print(variations)
