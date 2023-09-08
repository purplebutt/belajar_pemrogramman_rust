def calc(mode:str, start:int, *args) -> int:
    result = start
    for n in args:
        if mode == "add":
            result += n
        if mode == "mul":
            result *= n
    return result


if __name__ == "__main__":
    result = calc("add", 0, 1,2,3,4,5)
    print(f"total: {result}")
    result = calc("mul", 5, 1,2,3,4,5)
    print(f"product: {result}")

