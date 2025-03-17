import psycopg2
from datetime import datetime

class DatabaseManager:
    """A class to handle PostgreSQL database operations."""

    def __init__(self, dbname, user, password, host="127.0.0.1", port="5432"):
        """Initialize the database connection."""
        self.db_config = {
            "dbname": dbname,
            "user": user,
            "password": password,
            "host": host,
            "port": port
        }

    def _get_connection(self):
        """Create and return a new database connection."""
        return psycopg2.connect(**self.db_config)

    def insert_batch_execution(self, name, status):
        """Insert a new batch execution and return the inserted row ID."""
        sql = """
            INSERT INTO batch_executions (name, start_date, status) 
            VALUES (%s, %s, %s) 
            RETURNING id;
        """
        values = (name, datetime.now(), status)

        with self._get_connection() as conn:
            with conn.cursor() as cur:
                cur.execute(sql, values)
                inserted_id = cur.fetchone()[0]

        print("Inserted row ID:", inserted_id)
        return inserted_id

    def update_batch_execution_status(self, execution_id, new_status):
        """Update the status and end_date of a batch execution by ID."""
        sql = """
            UPDATE batch_executions 
            SET status = %s, end_date = %s 
            WHERE id = %s
            RETURNING id;
        """
        values = (new_status, datetime.now(), execution_id)

        with self._get_connection() as conn:
            with conn.cursor() as cur:
                cur.execute(sql, values)
                updated_id = cur.fetchone()

        if updated_id:
            print("Updated row ID:", updated_id[0])
            return updated_id[0]
        else:
            print("No row updated.")
            return None

# Usage Example
if __name__ == "__main__":
    db = DatabaseManager(
        dbname="roamdb",
        user="myuser",
        password="mypassword",
        host="127.0.0.1",
        port="5432"
    )

    # Insert a new batch execution
    inserted_id = db.insert_batch_execution("value1", "value2")

    # Update the batch execution status
    db.update_batch_execution_status(inserted_id, "new_status")
