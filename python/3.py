def lpf(x):
    lpf = 2
    while (x > lpf):
        if (x % lpf == 0):
            x = x / lpf
            lpf = 2
        else:
            lpf += 1
    print("Largest Prime Factor: %d" % (lpf))


def main():
    x = 600851475143
    lpf(x)
    return 0


if __name__ == '__main__':
    main()
