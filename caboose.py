#!/usr/bin/env python3

from tqdm import tqdm

def is_prime(n):
    if n < 2:
         return False;
    if n % 2 == 0:
         return n == 2
    k = 3
    while k*k <= n:
         if n % k == 0:
             return False
         k += 2
    return True

def is_caboose(m):
    for n in range(0, m):
        x = n**2 - n + m
        if not is_prime(x):
            return False
    return True

for n in tqdm(range(0, 1_000_000)):
    p = is_prime(n)
    if p:
        c = is_caboose(n)
        if c:
            print(f"{n}\t{p}\t{c}")
