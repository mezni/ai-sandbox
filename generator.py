import random
from datetime import datetime, timedelta


def generate_file_name():
    """
    generate file name
    """
    prefix = "ffff"
    name = "2019"
    suffix = ".csv"
    file_name = prefix + name + suffix
    return file_name


def generate_random_ip():
    """
    generate ip
    """
    return ".".join(str(random.randint(0, 255)) for _ in range(4))


def generate_time_boundries(date_max, date_interval):
    """
    generate time boundries
    """
    time_max = datetime.strptime(date_max, "%Y-%m-%d %H:%M:%S")  # .strftime("%S")
    time_min = time_max - timedelta(minutes=date_interval)
    return time_min, time_max
