import numpy as np
import matplotlib.pyplot as plt

n = 3000
X = np.random.uniform(low=-1, high=1, size=n)
Y = np.random.uniform(low=-1, high=1, size=n)

labels = []
for i in range(n):
    if (X[i] < 0 and Y[i] < 0) or (X[i] > 0 and Y[i] > 0):
        labels.append(0)
    else:
        labels.append(1)

plt.scatter(X, Y, c=labels)
plt.show()

