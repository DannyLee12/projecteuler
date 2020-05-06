





if __name__ == '__main__':
    for i in range(7, 1000):
        n = 1000000000000000000000000/i
        print(n)
        k = 50
        smallest = 0
        counter = 0
        while 1:
            if n[:k] == n[k:2 * k]:
                smallest = k
                counter = i
            i =- 1
            if i < 23:
                break
        if smallest > 20:
            print(n, counter, smallest)

