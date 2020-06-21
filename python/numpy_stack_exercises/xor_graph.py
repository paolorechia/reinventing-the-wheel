""" Xor Graph """

import numpy as np
import matplotlib.pyplot as plt

N = 3000
X = np.random.uniform(low=-1, high=1, size=N)
Y = np.random.uniform(low=-1, high=1, size=N)

labels = []
for i in range(N):
    if (X[i] < 0 and Y[i] < 0) or (X[i] > 0 and Y[i] > 0):
        labels.append(0)
    else:
        labels.append(1)

plt.scatter(X, Y, c=labels)
plt.show()
