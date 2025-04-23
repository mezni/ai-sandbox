import pandas as pd

#Oman,Omantel,968,99,96899

df = pd.read_csv('test.csv', delimiter='|')

df.rename(columns={df.columns[0]: "prefix"}, inplace=True)
df.rename(columns={df.columns[1]: "operator"}, inplace=True)

df.columns = df.columns.str.strip().str.lower()
df['country']='France'
df['cc']='33'
df['ndc'] = df['prefix'].astype(str).str[2:]

df = df[['country','operator','cc','ndc','prefix']]
df.to_csv('result.csv', index=False)
print (df.head())