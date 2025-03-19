import pandas as pd

def gen_cfg_countries():
    df = pd.read_csv('INPUT/country-codes.csv')

    df_countries = df.rename(columns={'Dial': 'country_code', 'ISO3166-1-Alpha-2': 'alpha2', 'ISO3166-1-Alpha-3': 'alpha3',
    'Region Name':'region_name','Sub-region Name':'sub_region_name','Intermediate Region Name':'intermediate_region_name'
    })

    df_countries = df_countries[['country_code','alpha2','alpha3','official_name_en','official_name_fr','region_name','sub_region_name','intermediate_region_name']]
    df_countries['country_code']=df_countries['country_code'].str.replace('-','')

    print (df_countries.head())
    df_countries.to_csv('INPUT/cfg_countries.csv', index=False)


def gen_clean_carriers():
    df = pd.read_excel('INPUT/ContryCode.xls')
    df['COUNTRY_CODE'] = pd.to_numeric(df['COUNTRY_CODE'], errors='coerce')
    df['COUNTRY_CODE'] = df['COUNTRY_CODE'].astype('Int64').astype(str)

    df = df.rename(columns={'PLMNNAME': 'carrier_name', 'COUNTRY': 'country_name', 'COUNTRY_CODE':'country_code', 'NATIONAL_DESTINATION_CODE':'national_destination_code', 'ID':'carrier_id'})
    df.to_csv('INPUT/cfg_carriers.csv', index=False)
    print (df.head())

#gen_clean_carriers()
df_carriers = pd.read_csv('INPUT/cfg_carriers.csv')
df_countries = pd.read_csv('INPUT/cfg_countries.csv')
df_carriers['country_code'] = pd.to_numeric(df_carriers['country_code'], errors='coerce')
df_carriers['country_code'] = df_carriers['country_code'].astype('Int64').astype(str)
#df_countries['country_code'] = df_countries['country_code'].astype('Int64')
print (df_carriers.head())
print (df_countries.head())

merged_df = pd.merge(df_carriers, df_countries, left_on='country_code', right_on='country_code', how='left')
print (len(merged_df))
df = merged_df[pd.isna(merged_df['official_name_fr'])]
print (len(df))
print (df.head())
merged_df.to_csv('temp.csv', index=False)