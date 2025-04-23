import pandas as pd 

#tadig,plmn,mcc,mnc,t2g,t3g,lte,operator,brand,country_iso

df = pd.read_csv('mcc-mnc.csv', sep=';')
print (len(df))
print (df.head())
df = df[df["TADIG"].notna()]
print (len(df))

df = df.rename(columns={
    "TADIG": "tadig",
    "PLMN": "plmn",
    "MCC": "mcc",
    "MNC": "mnc",
    "Brand": "operator",
    "Operator": "brand",
    "ISO": "country_iso",    
})

df["t2g"] = df["Bands"].str.contains("UMTS", na=False).map({True: "Yes", False: None})
df["t3g"] = df["Bands"].str.contains("GSM", na=False).map({True: "Yes", False: None})
df["lte"] = df["Bands"].str.contains("LTE", na=False).map({True: "Yes", False: None})

df = df[['tadig','plmn','mcc','mnc','t2g','t3g','lte','operator','brand','country_iso']]
df["operator"] = df["operator"].fillna(df["brand"])
df = df.sort_values(by=["country_iso", "plmn"])

print (df.head())   


df.to_csv('test.csv', index=False)