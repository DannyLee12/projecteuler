
if __name__ == '__main__':
    a = set()
    for x in range(2, 101):
        for y in range(2, 101):
            a.add(x**y)

    print(len(a))
