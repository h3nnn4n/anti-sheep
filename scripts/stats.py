import sys
import scipy.stats

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

from statsmodels.stats.multicomp import pairwise_tukeyhsd


datafile = sys.argv[1]
data = pd.read_csv(datafile)

f, p = scipy.stats.f_oneway(data['forward'], data['reverse'], data['double_headed'])

alpha = 0.05

if p < alpha:
    columns = data.columns

    for i in range(len(columns)):
        for j in range(i + 1, len(columns)):
            c1 = columns[i]
            c2 = columns[j]
            result = scipy.stats.wilcoxon(data[c1], data[c2])[1]
            if result >= alpha:
                print("p_value %6.4f: %15s = %15s" % (result, c1, c2))
            else:
                if np.mean(data[c1]) < np.mean(data[c2]):
                    print("p_value %6.4f: %15s > %15s" % (result, c1, c2))
                else:
                    print("p_value %6.4f: %15s < %15s" % (result, c1, c2))
