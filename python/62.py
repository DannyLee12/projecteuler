from collections import defaultdict


if __name__ == '__main__':
    d = defaultdict(int)
    for i in range(10000, 0, -1):
        val = "".join(sorted(list(str(i*i*i))))
        d[val] += 1
        if d[val] == 5:
            print(i)
            print(i*i*i)
