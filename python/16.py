def sum_(value):
    total = 0
    print(2**value)
    for char in str(2**value):
        total += int(char)
    print(total)


if __name__ == '__main__':
    sum_(1000)
