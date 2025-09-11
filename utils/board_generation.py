""" Simple tool to generate bit board configurations from hex numbers and hex numbers from bit board configurations """

def matrix_to_u64(matrix):
    value = 0
    for row in matrix:
        for bit in row:
            value = (value << 1) | int(bit)
    return value

def u64_to_matrix(number):
    bits = f"{number:064b}"  
    return [bits[i:i+8] for i in range(0, 64, 8)]

if __name__ == "__main__":
    matrix = [
        "00000000",
        "00000000",
        "00000000",
        "00000010",
        "00000000",
        "00000000",
        "00000000",
        "00000000",
    ]

    number = matrix_to_u64(matrix)
    print("From matrix to number")
    print(f"Binary: {number:064b}")
    print(f"Hex   : 0x{number:016X}")

    print("\nFrom number to matrix")
    restored = u64_to_matrix(0x804000100804)
    for row in restored:
        print(row)
