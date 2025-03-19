import pandas as pd

def gen_cfg_countries():
    df = pd.read_csv('INPUT/country-codes.csv')

    df_countries = df.rename(columns={'Dial': 'country_code', 'ISO3166-1-Alpha-2': 'alpha2', 'ISO3166-1-Alpha-3': 'alpha3',
    'Region Name':'region_name','Sub-region Name':'sub_region_name','Intermediate Region Name':'intermediate_region_name'
    })

    df_countries = df_countries[['country_code','alpha2','alpha3','official_name_en','official_name_fr','region_name','sub_region_name','intermediate_region_name']]
    df_countries['country_code']=df_countries['country_code'].str.replace('-','')

    print (df_countries.head())
    df_countries.to_csv('OUTPUT/cfg_countries.csv', index=False)

#gen_cfg_countries()
def gen_clean_carriers():
    df = pd.read_excel('INPUT/ContryCode.xls')
    df['COUNTRY_CODE'] = pd.to_numeric(df['COUNTRY_CODE'], errors='coerce')
    df['COUNTRY_CODE'] = df['COUNTRY_CODE'].astype('Int64').astype(str)

    df = df.rename(columns={'PLMNNAME': 'carrier_name', 'COUNTRY': 'country_name', 'COUNTRY_CODE':'country_code', 'NATIONAL_DESTINATION_CODE':'national_destination_code', 'ID':'carrier_id'})
    df.to_csv('OUTPUT/cfg_carriers.csv', index=False)
    print (df.head())
#gen_clean_carriers()

def gen_countries_sql():
    table_name = 'dim_countries'
    filename = 'ins_countries.sql'
    df = pd.read_csv('OUTPUT/cfg_countries.csv')

    insert_statements = []
    for index, row in df.iterrows():
        columns = ', '.join(row.index)
        values = ', '.join(f"'{str(value)}'" if isinstance(value, str) else str(value) for value in row.values) 
        insert_statement = f"INSERT INTO {table_name} ({columns}) VALUES ({values});"
        insert_statements.append(insert_statement)

    sql_script = '\n'.join(insert_statements)

    
    with open(filename, 'w') as file:
        file.write(sql_script)
#gen_countries_sql()




def gen_carriers_sql():
    table_name = 'dim_carriers'
    filename = 'ins_carriers_tmp.sql'
    df_carriers = pd.read_csv('OUTPUT/cfg_carriers.csv')
    df_countries = pd.read_csv('OUTPUT/cfg_countries.csv')
    df_carriers['country_code'] = pd.to_numeric(df_carriers['country_code'], errors='coerce')
    df_carriers['country_code'] = df_carriers['country_code'].astype('Int64').astype(str)
    #df_countries['country_code'] = df_countries['country_code'].astype('Int64')
    merged_df = pd.merge(df_carriers, df_countries, left_on='country_name', right_on='official_name_en', how='left')
    df = merged_df[merged_df['official_name_fr'].notna()]
    df = df[['carrier_name', 'country_code','national_destination_code','official_name_en']]
    insert_statements = []
    for index, row in df.iterrows():
        carrier_name = row['carrier_name']
        country_code = row['country_code']
        national_destination_code = row['national_destination_code']
        official_name_en = row['official_name_en']
        
        insert_statement = (
            f"INSERT INTO {table_name} (carrier_name, country_code, national_destination_code, country_id) "
            f"VALUES ('{carrier_name}', '{country_code}', '{national_destination_code}',"
            f"(SELECT id FROM dim_countries WHERE name = '{official_name_en}'));"
        )
        insert_statements.append(insert_statement)
    sql_script = '\n'.join(insert_statements)
    
    with open(filename, 'w') as file:
        file.write(sql_script)
#gen_carriers_sql()

df_carriers = pd.read_csv('OUTPUT/cfg_carriers.csv')
df_countries = pd.read_csv('OUTPUT/cfg_countries.csv')
df_carriers['country_code'] = pd.to_numeric(df_carriers['country_code'], errors='coerce')
df_carriers['country_code'] = df_carriers['country_code'].astype('Int64').astype(str)
#df_countries['country_code'] = df_countries['country_code'].astype('Int64')
merged_df = pd.merge(df_carriers, df_countries, left_on='country_name', right_on='official_name_en', how='left')
#df = merged_df[merged_df['official_name_fr'].notna()]
print (len(df_carriers))
print (len(df_countries))
print (len(merged_df))
df = merged_df[pd.isnull(merged_df['official_name_fr'])]
print (df.head(40))
print (len(df))