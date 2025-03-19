import pandas as pd

columns_pays = ['Column1', 'Column2', 'country_alpha2', 'country_alph3', 'country_name_fr', 'country_name_en']

df = pd.read_csv('sql-pays.csv', header=None)
df.columns = columns_pays

df_countries = df[['country_name_en', 'country_name_fr', 'country_alpha2', 'country_alph3']] 
df_countries['country_name_jn'] = df_countries['country_name_en'].str.replace(r'\s+', '', regex=True)

print (df_countries.head())

df = pd.read_excel('ContryCode.xls')

df_carriers = df.rename(columns={'PLMNNAME': 'carrier_name', 'COUNTRY': 'country_name'})
df_carriers.to_csv("carriers.csv")
#df['COUNTRY_CODE'] = df['COUNTRY_CODE'].astype(int)
#df['NATIONAL_DESTINATION_CODE'] = df['NATIONAL_DESTINATION_CODE'].astype(int)

print (df_carriers.head())

merged_df = pd.merge(df_carriers, df_countries, left_on='country_name', right_on='country_name_jn', how='left')



df = merged_df[['country_name_en', 'country_name_fr', 'country_alpha2', 'country_alph3', 'COUNTRY_CODE']]
#df.to_csv("test.csv")
df = df.drop_duplicates()

print (df.head())


records = df.to_records(index=False)

print(records)