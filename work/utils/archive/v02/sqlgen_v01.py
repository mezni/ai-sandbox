import pandas as pd

columns_pays = ['Column1', 'Column2', 'country_alpha2', 'country_alpha3', 'country_name_fr', 'country_name_en']

df = pd.read_csv('INPUT/sql-pays.csv', header=None)
df.columns = columns_pays

df_countries = df[['country_name_en', 'country_name_fr', 'country_alpha2', 'country_alpha3']] 
df_countries['country_name'] = df_countries['country_name_en']

print (df_countries.head())


df_countries.to_csv('countries.csv', index=False)


df = pd.read_csv('countries.csv')
df.to_csv('test.csv', index=False)


df = pd.read_excel('INPUT/ContryCode.xls')
print (df.head())