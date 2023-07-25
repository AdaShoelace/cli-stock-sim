#!/bin/python3

from argparse import ArgumentParser
import random
import string

VOWELS = "eauiyo"

def is_vowel(char):
    if char in VOWELS:
        return True
    else:
        return False

def split_company_name(company_name):
    ret = []
    split_list = company_name.split(' ')
    for i in range(len(split_list)):
        ret.append(list(split_list[i]))
    return ret

def calculate_range_len(company_name_list):
    len_list = [len(x) for x in company_name_list]
    len_list.sort()
    return len_list[0]

def generate_variations(company_name, num_variations, hamming_distance):
    variations = []
    for _ in range(num_variations):
        variation = split_company_name(company_name)
        range_len = calculate_range_len(variation)
        indices_to_change = random.sample(range(range_len), hamming_distance)

        for idx in indices_to_change:
            for word in range(len(variation)):
                char = variation[word][idx]
                if is_vowel(char):
                    variation[word][idx] = random.choice(''.join(c for c in VOWELS if char != c))
                else:
                    variation[word][idx] = random.choice(string.ascii_lowercase)
        new = ""
        for l in variation:
            for word in l:
                for c in word:
                    new += c
            new += ' '
        #variations.append(''.join(variation))
        variations.append(''.join(new.strip()))
    return variations

# Example usage
company_name = 'Mercedes'
num_variations = 1
hamming_distance = 1

variations = generate_variations(company_name, num_variations, hamming_distance)
print(variations)

if __name__=="__main__":
    parser = ArgumentParser()

    parser.add_argument("-o", "--output", dest="output", help="Write output to file", metavar="FILE")
    parser.add_argument("-i", "--input", help="Input file", metavar="FILE")
    args = parser.parse_args()

    infile = vars(args)["input"]
    outfile = vars(args)["output"]
    with open(infile) as f:
        for line in f.readlines():
            variation = generate_variations(line.strip(), 1, 1)[0]
            print(f'{line.strip()} => {variation}')
            with open(outfile, "a") as outf:
                outf.write(f"{variation}\n")
