import pandas as pd
import psycopg2
from loguru import logger

# Postgres connection settings
PG_HOST = 'localhost'
PG_PORT = 5432
PG_DBNAME = 'mydatabase'
PG_USER = 'myuser'
PG_PASSWORD = 'mypassword'

# CSV file settings
CSV_FILE = 'data.csv'

def read_csv(file_path):
    """Reads a CSV file into a Pandas DataFrame"""
    try:
        df = pd.read_csv(file_path)
        return df
    except Exception as e:
        logger.error(f"Error reading CSV file: {e}")
        return None

def transform_data(df):
    """Transforms the data (e.g., converts data types)"""
    # Example transformation: convert 'date' column to datetime
    df['date'] = pd.to_datetime(df['date'])
    return df

def load_to_postgres(df):
    """Loads the transformed data into a Postgres database"""
    try:
        conn = psycopg2.connect(
            host=PG_HOST,
            port=PG_PORT,
            dbname=PG_DBNAME,
            user=PG_USER,
            password=PG_PASSWORD
        )
        cur = conn.cursor()
        cur.executemany("INSERT INTO mytable (name, date) VALUES (%s, %s)", df.values.tolist())
        conn.commit()
        cur.close()
        conn.close()
        logger.info("Data loaded into Postgres")
    except Exception as e:
        logger.error(f"Error loading data into Postgres: {e}")

def main():
    logger.info("Starting microservice")
    df = read_csv(CSV_FILE)
    if df is not None:
        df = transform_data(df)
        load_to_postgres(df)
    logger.info("Microservice complete")

if __name__ == "__main__":
    main()