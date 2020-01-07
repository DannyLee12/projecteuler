"""
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.
"""


def sum_3_5(range_: int) -> int:
    """Sum multiples of 3 and 5"""
    total = 0
    for x in range(1, range_):
        if x % 3 == 0 or x % 5 == 0:
            total += x
    return total


if __name__ == '__main__':
    assert sum_3_5(10) == 23
    print(sum_3_5(1000))
