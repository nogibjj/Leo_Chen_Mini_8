import time
import psutil
from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import query


def measure_query():
    start_time = time.perf_counter()
    process = psutil.Process()
    initial_memory = process.memory_info().rss / 1024

    query()

    elapsed_time = time.perf_counter() - start_time
    final_memory = process.memory_info().rss / 1024
    final_cpu = process.cpu_percent(interval=0.1)

    print(f"Query function completed in {elapsed_time:.5f} seconds")
    print(f"Memory usage: {final_memory - initial_memory:.2f} KB")
    print(f"CPU usage: {final_cpu:.2f}%")


# Extract
print("Extracting data...")
extract()

# Transform and load
print("Loading data...")
load()

# Query
print("Querying data...")
measure_query()

print("All done!")