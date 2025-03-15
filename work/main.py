from abc import ABC, abstractmethod
import pandas as pd

class Source(ABC):
    @abstractmethod
    def extract(self):
        pass

class FileSource(Source):
    def __init__(self, file_path):
        self.file_path = file_path

    def extract(self):
        return pd.read_csv(self.file_path), None



class ETLPipeline:
    def __init__(self, source: Source):
        self.source = source


    def run(self):
        extracted_data, error = self.source.extract()

file_path = '../../DATA/data.csv'
source = FileSource(file_path)


pipeline = ETLPipeline(source)
pipeline.run()
