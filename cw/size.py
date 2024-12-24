
import matplotlib.pyplot as plt

# Data
types = ['json', 'bit packed 18 bit', 'gRPC']
sizes_bytes = [4605963/40, 655360/40,  1048576/40]

# Convert sizes to megabytes (MB)
sizes_mb = [size / (1024) for size in sizes_bytes]

# Create bar chart
plt.figure(figsize=(10, 6))
plt.bar(types, sizes_mb)

# Add labels and title
plt.xlabel('Type of encoding')
plt.ylabel('Size (KB)')
plt.title('Size Comparison for data transfer of a sparse grid (~6500 cells) in MB')
plt.ylim(0, max(sizes_mb) * 1.1)  # Add some space above the tallest bar

# Add size labels on top of the bars
for i, size in enumerate(sizes_mb):
    plt.text(i, size + 0.05, f'{size:.2f} kB', ha='center', va='bottom')

# Save the plot as a PNG file
plt.savefig('size_comparison.png')

# Show the plot
plt.show()