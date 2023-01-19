# usr/bin/python3
import numpy as np
import matplotlib.pyplot as plt


def plotRes(in_array):  # plots value matrix
    x = np.arange(1, 10, 1)  # dealer axis
    y = np.arange(0, 21, 1)  # player axis
    x, y = np.meshgrid(x, y)
    z = in_array[x, y]  # self.V[x, y]  # hit?
    fig = plt.figure()
    ax = plt.axes(projection='3d')
    surf = ax.plot_surface(x, y, z, cmap=plt.cm.cividis)
    plt.show()


if __name__ == '__main__':
    v = np.load('npy_res/v.npy')
    plotRes(v)

    n_3d = np.load('npy_res/n.npy')
    n = np.sum(n_3d, axis=2)
    # for d in range(0, 11):
    #     for p in range(0, 23):
    #         n[d, p] = n_3d[d, p, 0] + n[d, p, 1]
    # print(n.shape)
    plotRes(n)
