import pandas as pd

# URL of the Wikipedia page with mobile telephone prefixes
url = "https://en.wikipedia.org/wiki/List_of_mobile_telephone_prefixes_by_country"

# Read all tables from the page
tables = pd.read_html(url)

# Loop through tables and find the one containing international prefixes
# This assumes the "International prefixes" table has a specific header or pattern.
for i, table in enumerate(tables):
    # Convert columns to string and check if "International prefixes" is present
    if "International prefixes" in table.columns.astype(str).str.contains("International", case=False, na=False).values:
        # Assuming we found the correct table, let's save it
        df = table
        df.to_csv("international_mobile_prefixes.csv", index=False)
        print(f"Saved table {i} to 'international_mobile_prefixes.csv'")
        break
