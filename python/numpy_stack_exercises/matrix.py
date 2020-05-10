import sys

from random import randint
from pprint import pprint

import numpy as np
from time import perf_counter

def generate_random_matrix(n=100):
    rows = []
    for i in range(n):
        cols = []
        for j in range(n):
            cols.append(randint(0, 5))
        rows.append(cols)
    return rows


def pure_python_multiplication(A, B, n):
    # iterate through A lines
    C = []
    for i in range(n):
        # iterate through A columns
        row = []
        for j in range(n):
            sum_ = 0.0
            # iterate through B lines
            for k in range(n):
                sum_ += A[i][k] * B[k][j]
            row.append(sum_)
        C.append(row)
    return C


def numpy_multiplication(A, B):
    # print("num input:")
    # print(A)
    # print(B)
    return A.dot(B)


def print_matrix(A, n):
    print("----")
    for i in range(n):
        sys.stdout.write("[")
        for j in range(n):
            sys.stdout.write(str(A[i][j]) + " ")
        print("]")
    print("----")


if __name__ == "__main__":
    n = int(sys.argv[1]) if len(sys.argv) >= 2 else 2
    A = generate_random_matrix(n)
    B = generate_random_matrix(n)
    pure_start = perf_counter()
    pure = pure_python_multiplication(A, B, n)

    elapsed_pure_time = perf_counter() - pure_start

    num_start = perf_counter()
    num = numpy_multiplication(np.array(A), np.array(B))
    elapsed_num_time = perf_counter() - num_start

    # print("---Generate---")
    # print_matrix(A, n)
    # print_matrix(B, n)
    # print("---Pure---")
    # print(pure)
    # print("---Numpy---")
    # print(num)

    for i in range(n):
        for j in range(n):
            assert int(pure[i][j]) == int(num[i][j])

