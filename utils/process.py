import os
from itertools import product
import pandas as pd
import numpy as np

input_operators_dir = 'INPUT/operators'
input_ranges_dir = 'INPUT/ranges'

def flatten_range1(range_str):
    if '[' in range_str and ']' in range_str:
        prefix, range_part = range_str.split('[')
        range_values = range_part.strip(']')
        start, end = map(int, range_values.split('-'))
        return [f"{prefix}{i}" for i in range(start, end + 1)]
    else:
        return [range_str]  


def expand_part(part):
    expanded_values = []
    i = 0
    while i < len(part):
        if i + 2 < len(part) and part[i+1] == '-':
            # Handle ranges
            start = int(part[i])
            end = int(part[i+2])
            expanded_values.extend(str(x) for x in range(start, end + 1))
            i += 3
        else:
            # Single digit
            expanded_values.append(part[i])
            i += 1
    return expanded_values

def flatten_range(range_str):
    parts = []
    while '[' in range_str and ']' in range_str:
        prefix, rest = range_str.split('[', 1)
        range_part, range_str = rest.split(']', 1)
        expanded = expand_part(range_part)
        parts.append([f"{prefix}{x}" for x in expanded])

    # Append any remaining part
    if range_str:
        parts.append([range_str])

    # Cartesian product of all parts
    return [''.join(p) for p in product(*parts)]



def get_operators_df(path):
    df = pd.read_csv(path, sep=';')
    df.columns = df.columns.str.strip()
    df = df.map(lambda x: x.strip() if isinstance(x, str) else x)
    df = df.rename(columns={'Name:en': 'Name'})
    df['Name'] = df['Name'].str.replace('"', '', regex=False)
    return df

def get_rangers_df(path):
    filename = os.path.basename(path)
    filebase, file_extension = os.path.splitext(filename)
    df = pd.read_csv(path, sep=';')
    df.columns = df.columns.str.strip()
    df = df.map(lambda x: x.strip() if isinstance(x, str) else x)

    df['CountryCode'] = filebase
    df = df[df['Type'].isin(['MOBILE', 'FIXED_LINE_OR_MOBILE'])]
    df = df [['CountryCode','Prefix','Length','Operator','Regions']]

    df['Operator'] = df['Operator'].str.replace('"', '', regex=False)
    df['Regions'] = df['Regions'].str.replace('"', '', regex=False)
    df['Prefix'] = df['Prefix'].astype(str)
    df['flattened'] = df['Prefix'].apply(flatten_range)
    df = df.explode('flattened').reset_index(drop=True)
    df = df [['CountryCode','flattened','Length','Operator','Regions']]
    df = df.rename(columns={'flattened': 'Prefix'})
    df = df [['CountryCode','Prefix','Length','Operator','Regions']]
    return df

def main():
    global_df = pd.DataFrame()
    for filename in os.listdir(input_operators_dir):
        print (filename)
        operators_file_path_exists=False
        ranges_file_path_exists=False
        operators_file_path = os.path.join(input_operators_dir, filename)
        if os.path.isfile(operators_file_path):
            operators_file_path_exists=True
            filename = os.path.basename(operators_file_path)
            ranges_file_path = os.path.join(input_ranges_dir, filename)
            if os.path.isfile(ranges_file_path):
                ranges_file_path_exists=True
        
        if operators_file_path_exists:
            operators_df = get_operators_df(operators_file_path)

        if ranges_file_path_exists:
            ranges_df = get_rangers_df(ranges_file_path)      
            if operators_file_path_exists:
                join_df = pd.merge(ranges_df, operators_df, left_on='Operator', right_on='Id', how='left')
                join_df = join_df.rename(columns={'Operator': 'OperatorId','Name': 'OperatorName'}) 
            else:
                join_df=ranges_df
                join_df = join_df.rename(columns={'Operator': 'OperatorId'}) 
                join_df['join_df']=np.nan
            join_df=join_df[['CountryCode','Prefix','Length','OperatorId','OperatorName','Regions']]
            global_df = pd.concat([global_df, join_df], ignore_index=True)

    print (global_df.head())
    global_df.to_csv('tmp.csv',index=False)

main()