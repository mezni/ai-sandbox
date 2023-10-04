from datetime import datetime
from generator import generate_file_name, generate_time_boundries


def setup_function(function):
    print(f" Running Setup: {function.__name__}")
    function.date_interval = 5
    function.date_max = "2023-10-23 20:00:00"


def teardown_function(function):
    print(f" Running Teardown: {function.__name__}")
    function.date_interval = 5
    function.date_max = "2023-10-23 20:00:00"


def test_generate_time_boundries():
    assert generate_time_boundries(
        test_generate_time_boundries.date_max,
        test_generate_time_boundries.date_interval,
    ) == (datetime(2023, 10, 23, 19, 55), datetime(2023, 10, 23, 20, 0))


def test_generate_file_name():
    assert generate_file_name() == "ffff2019.csv"
