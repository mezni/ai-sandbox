import pandas as pd

class FileReader:
    def __init__(self, file_type, file_path):
        self.file_type = file_type  
        self.file_path = file_path    

    def extract(self):
        try:
            data = pd.read_csv(self.file_path)
            return data, None  
        except pd.errors.EmptyDataError:
            return None, "Error: CSV file is empty."
        except FileNotFoundError:
            return None, f"Error: File not found at {self.file_path}."
        except pd.errors.ParserError:
            return None, "Error: CSV file is incorrectly formatted."
        except Exception as e:
            return None, f"Unexpected error: {str(e)}"


file_path = '../../DATA/data.csv'
reader = FileReader("csv", file_path)
data, error = reader.extract()

if error:
    print(error)
else:
    print(data.head())  
