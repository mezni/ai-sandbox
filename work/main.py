import pandas as pd

class FileReader:
    def __init__(self, file_type, file_path):
        self.file_type = file_type  
        self.file_path = file_path    

    def extract(self):
        if self.file_type not in ["roam_in", "roam_out"]:  
            return None, "Error: Unsupported file type."

# Example usage
file_path = '../../DATA/data.csv'
reader = FileReader("roam_in", file_path)
data, error = reader.extract()

if error:
    print(error)
else:
    print(data.head())  # Print first few rows
