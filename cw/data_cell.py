import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

def parse_time(time_str):
    """
    Parses a time string with units and converts it to microseconds.
    
    Args:
        time_str (str): Time string with units (e.g., '123us', '456ms')
    
    Returns:
        float: Time in microseconds
    """
    if time_str.endswith('us'):
        return float(time_str[:-2])  # Already in microseconds
    elif time_str.endswith('ms'):
        return float(time_str[:-2]) * 1e3  # Convert milliseconds to microseconds
    elif time_str.endswith('s'):
        return float(time_str[:-1]) * 1e6  # Convert seconds to microseconds
    else:
        raise ValueError(f"Unknown time unit in {time_str}")

def analyze_decode_times(file_paths):
    """
    Analyzes decode operation times for each CSV file.
    
    Args:
        file_paths (list): List of paths to CSV files
    
    Returns:
        dict: Dictionary with file paths as keys and (min_time, mean_time, max_time) as values
    """
    results = {}
    
    # Process each file
    for file_path in file_paths:
        try:
            # Read CSV file
            df = pd.read_csv(file_path)
            
            # Filter for 'Decode' operations and get their times
            print(f"Columns in {file_path}: {df.columns.tolist()}")
            if 'Operation' in df.columns:
                decode_times = df[df['Operation'].str.strip() == 'Decode']['Time (seconds)'].values
                decode_times = [parse_time(time_str) for time_str in decode_times]
                
                # Convert to numpy array for calculations
                decode_times = np.array(decode_times)
                
                # Adjust times for approximately 7000 cells
                decode_times /= 40
                
                # Calculate statistics
                min_time = np.min(decode_times)
                mean_time = np.mean(decode_times)
                max_time = np.max(decode_times)
                
                # Store results
                results[file_path] = (min_time, mean_time, max_time)
            else:
                print(f"'Operation' column not found in {file_path}")
        except FileNotFoundError:
            print(f"Error: Could not find file {file_path}")
        except Exception as e:
            print(f"An error occurred while processing {file_path}: {str(e)}")
    
    return results

def plot_mean_times(results, output_path='decode_times_comparison_7000.png'):
    """
    Creates and saves a bar plot of mean decode times in microseconds.
    
    Args:
        results (dict): Dictionary with file paths and their statistics
        output_path (str): Path where to save the plot
    """
    # Extract file names and mean times, maintaining the order from file_paths
    file_names = [path.split('/')[-1].replace('_results.csv', '').replace('.csv', '').replace('_', ' ') for path in file_paths]
    mean_times = [results[path][1] for path in file_paths]  # stats[1] is mean_time

    # Create the plot
    plt.figure(figsize=(10, 6))
    bars = plt.bar(file_names, mean_times)
    
    # Customize the plot
    plt.title('Mean Decode Times Comparison for a Sparse Grid (~6500 Cells)')
    plt.xlabel('Type of encoding')
    plt.ylabel('Time (µs)')
    
    # Add value labels on top of each bar
    for bar in bars:
        height = bar.get_height()
        plt.text(bar.get_x() + bar.get_width()/2., height,
                f'{height:.2f}µs',
                ha='center', va='bottom')
    
    # Adjust layout and save
    plt.tight_layout()
    plt.savefig(output_path)
    print(f"Plot saved as {output_path}")

# Example usage with reordered file paths
file_paths = [
    './test_yar/json_results.csv',
    './decoder/bit_packed_18_bit_results.csv',
    './proto/gRPC_results.csv',
]

try:
    results = analyze_decode_times(file_paths)
    
    # Print statistics in the desired order
    print(f"\nStatistics for Decode operations for each file:")
    for file_path in file_paths:
        min_time, mean_time, max_time = results[file_path]
        print(f"\nFile: {file_path}")
        print(f"Minimum time: {min_time:.4f} microseconds")
        print(f"Mean time: {mean_time:.4f} microseconds")
        print(f"Maximum time: {max_time:.4f} microseconds")
    
    # Create and save the plot
    plot_mean_times(results)
    
except Exception as e:
    print(f"An error occurred: {str(e)}")