import numpy as np
import matplotlib.pyplot as plt
import pandas as pd
from math import sqrt


r1 = 5
r2 = 10


def cpfactory(r=10):
    def circle_point(y):
        x = np.sqrt(r*r - y*y)
        for i in range(len(x)):
            if np.random.uniform() > 0.5:
                x[i] *= -1 
            x[i] += np.random.uniform(low=-2, high=2)
        return x

    return circle_point


def build_circle(radius=10, points=500):
    Y = np.random.uniform(low=-radius, high=radius, size=points)
    X = np.random.uniform(low=-radius, high=radius, size=points)

    cp = cpfactory(radius)

    X2 = cp(Y)
    Y2 = cp(X)

    X = np.hstack([X2, X])
    Y = np.hstack([Y, Y2])
    return X, Y

X, Y = build_circle(points=500)
X2, Y2 = build_circle(radius=5, points=300)
plt.scatter(X, Y)
plt.scatter(X2, Y2)
plt.show()
