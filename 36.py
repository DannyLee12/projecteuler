def main():
    def is_pal(s: str) -> bool:
        return s == s[::-1]

    total = 0

    for i in range(1_000_001):
        if is_pal(str(i)) and is_pal(bin(i)[2:]):
            total += i

    print(total)


if __name__ == '__main__':
    main()