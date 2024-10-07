"""
ETL-Query script
"""

from mylib.extract import extract
from mylib.transform_load import load
from mylib.query import query

# Extract
print("Extracting data...")
extract()

# Transform and load
print("Loading data...")
load()

# Query
print("Querying data...")
query()

print("All done!")