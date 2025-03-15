from abc import ABC, abstractmethod
import pandas as pd
import sqlite3

# Abstract Base Classes
class Source(ABC):
    @abstractmethod
    def extract(self):
        pass

class Stages(ABC):
    @abstractmethod
    def transform(self, data):
        pass

class Sink(ABC):
    @abstractmethod
    def load(self, data):
        pass

# Concrete Implementations
class FileSource(Source):
    def __init__(self, file_path):
        self.file_path = file_path

    def extract(self):
        return pd.read_csv(self.file_path)

class Enrich(Stages):
    def transform(self, data):
        data['enriched_column'] = data['existing_column'] * 2
        return data

class DBLoader(Sink):
    def __init__(self, db_path):
        self.db_path = db_path

    def load(self, data):
        conn = sqlite3.connect(self.db_path)
        data.to_sql('transformed_data', conn, if_exists='replace', index=False)
        conn.close()

# ETL Pipeline
class ETLPipeline:
    def __init__(self, source: Source, stages: Stages, sink: Sink):
        self.source = source
        self.stages = stages
        self.sink = sink

    def run(self):
        extracted_data = self.source.extract()
        transformed_data = self.stages.transform(extracted_data)
        self.sink.load(transformed_data)

# Example Usage
source = FileSource('test_data/source_data.csv')
stages = Enrich()
sink = DBLoader('output.db')

pipeline = ETLPipeline(source, stages, sink)
pipeline.run()