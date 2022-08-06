import matplotlib.pyplot as plt
import pandas as pd

df = pd.read_csv('trace.csv')

time = df['time']
electron_positions = df['position']

plt.xlabel("Electron Position")
plt.ylabel("Time")
plt.title("An Electron in a Potential Well")
plt.plot(electron_positions, time)
plt.savefig('plots/electron-oscillating.png')
plt.show()
