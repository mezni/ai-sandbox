import pandas as pd


df_countries = pd.read_csv('INPUT/config_countries.csv')
df_carriers = pd.read_csv('INPUT/config_carriers.csv')

merged_df = pd.merge(df_carriers, df_countries, left_on='country_name', right_on='country_name', how='inner')
merged_df['country_code'] = pd.to_numeric(merged_df['country_code'], errors='coerce')
merged_df['country_code'] = merged_df['country_code'].astype('Int64')
merged_df = merged_df.drop_duplicates()

df = merged_df[['country_code', 'country_name_en', 'country_name_fr', 'country_alpha2', 'country_alph3']]
df['country_code'] = df['country_code'].astype(str) 
df = df.drop_duplicates()
print (df.head())

table_name = 'prm_countries'
insert_statements = []
for index, row in df.iterrows():
    columns = ', '.join(row.index)
    values = ', '.join(f"'{str(value)}'" if isinstance(value, str) else str(value) for value in row.values) 
    insert_statement = f"INSERT INTO {table_name} ({columns}) VALUES ({values});"
    insert_statements.append(insert_statement)

sql_script = '\n'.join(insert_statements)

filename = 'insert_statements1.sql'

# Write the SQL script to the file
with open(filename, 'w') as file:
    file.write(sql_script)

df = merged_df[['carrier_name', 'country_code', 'national_destination_code', 'country_name_en']]
df['country_code'] = df['country_code'].astype(str) 

table_name = 'prm_carriers'
insert_statements = []
for index, row in df.iterrows():
    carrier_name = row['carrier_name']
    country_code = row['country_code']
    national_destination_code = row['national_destination_code']
    country_name_en = row['country_name_en']
    
    insert_statement = (
        f"INSERT INTO {table_name} (carrier_name, country_code, national_destination_code, country_id) "
        f"VALUES ('{carrier_name}', '{country_code}', '{national_destination_code}',"
        f"(SELECT id FROM countries WHERE name = '{country_name_en}'));"
    )
    insert_statements.append(insert_statement)

sql_script = '\n'.join(insert_statements)

filename = 'insert_statements2.sql'

# Write the SQL script to the file
with open(filename, 'w') as file:
    file.write(sql_script)

