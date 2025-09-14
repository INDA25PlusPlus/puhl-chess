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
        "11101010",
        "10101001",
        "01001000",
        "10011001",
        "10010100",
        "01100000",
        "01010101",
        "01100001",
    ]
    matrix = [
        "00110100",
        "00001000",
        "00000000",
        "00000000",
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
    restored = u64_to_matrix(4112)
    for row in restored:
        print(row)
