import pandas as pd

columns_pays = ['Column1', 'Column2', 'alpha2', 'alph3', 'name_fr', 'name_en']

df = pd.read_csv('sql-pays.csv', header=None)
df.columns = columns_pays

df_countries = df[['name_en', 'name_fr', 'alpha2', 'alph3']] 

print (df_countries.head())