"""Functions used in preparing Guido's gorgeous lasagna.

Learn about Guido, the creator of the Python language:
https://en.wikipedia.org/wiki/Guido_van_Rossum

This is a module docstring, used to describe the functionality
of a module and its functions and/or classes.
"""

EXPECTED_BAKE_TIME = 40
PREPARATION_TIME = 2


def bake_time_remaining(elapsed_bake_time: int) -> int:
    """Calculate the bake time remaining.

    :param elapsed_bake_time: int - baking time already elapsed.
    :return: int - remaining bake time (in minutes) derived from 'EXPECTED_BAKE_TIME'.

    Define the EXPECTED_BAKE_TIME constant https://stackoverflow.com/a/2682752 that
    represents how many minutes the lasagna .

    should bake in the oven.

    According to your cookbook, the Lasagna should be in the oven for 40 minutes:
    """
    if elapsed_bake_time >= EXPECTED_BAKE_TIME:
        raise ValueError("Over cooked")
    return EXPECTED_BAKE_TIME - elapsed_bake_time


def preparation_time_in_minutes(number_of_layers: int) -> int:
    """Calculate the preparation time in minutes.

    :param number_of_layers: int - the number of layers in the lasagna.
    :return: int - preparation time in minutes derived from 'number_of_layers'

    Define the preparation_time_in_minutes() function https://docs.python.org/3/tutorial/controlflow.html#defining-functions that takes the number_of_layers you want to add to the lasagna as an argument and returns how many minutes you would
    spend making them.

    Assume each layer takes 2 minutes to prepare.
    """
    return number_of_layers * PREPARATION_TIME


def elapsed_time_in_minutes(number_of_layers: int, elapsed_bake_time: int) -> int:
    """Calculate the elapsed cooking time.

    :param number_of_layers: int - the number of layers in the lasagna.
    :param elapsed_bake_time: int - elapsed cooking time.
    :return: int - elapsed cooking time

    Define the elapsed_time_in_minutes() function that takes two parameters as arguments: number_of_layers (the number of   layers added to the lasagna) and elapsed_bake_time (the number of minutes the lasagna has been baking in the oven).     This function should return the total number of minutes you have been cooking, or the sum of your preparation time and  the time the lasagna has already spent baking in the oven.
    """
    return preparation_time_in_minutes(number_of_layers) + elapsed_bake_time
