import pandas as pd
import os

class FileReader:
    def __init__(self, file_type, file_path):
        self.file_type = file_type  
        self.file_path = file_path    

    def extract(self):
        # Check if file type is supported
        if self.file_type not in ["roam_in", "roam_out"]:  
            return None, "Error: Unsupported file type."

        # Check if file exists
        if not os.path.exists(self.file_path):
            return None, f"Error: File not found at {self.file_path}."

        # Delegate to the appropriate extraction method
        if self.file_type == "roam_in":  
            return self._extract_roam_in()
        elif self.file_type == "roam_out":
            return self._extract_roam_out()   

    def _extract_roam_in(self):
        """ Reads and processes roam_in files. """
        try:
            data = pd.read_csv(self.file_path)
            return data, None  
        except pd.errors.EmptyDataError:
            return None, "Error: CSV file is empty."
        except pd.errors.ParserError:
            return None, "Error: CSV file is incorrectly formatted."
        except Exception as e:
            return None, f"Unexpected error: {str(e)}"

    def _extract_roam_out(self):
        """ Reads and processes roam_out files. Modify if format differs. """
        try:
            data = pd.read_csv(self.file_path)  # Modify if different format
            return data, None  
        except pd.errors.EmptyDataError:
            return None, "Error: CSV file is empty."
        except pd.errors.ParserError:
            return None, "Error: CSV file is incorrectly formatted."
        except Exception as e:
            return None, f"Unexpected error: {str(e)}"

# Example usage
file_path = '../../DATA/data.csv'
reader = FileReader("roam_in", file_path)
data, error = reader.extract()

if error:
    print(error)
else:
    print(data.head())  
