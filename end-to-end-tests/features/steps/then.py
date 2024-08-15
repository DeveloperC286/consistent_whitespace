from behave import then

from utilities import execute_consistent_whitespace
from assertions import *


@then('all files are consistent.')
def assert_all_files_consistent(context):
    # When
    result = execute_consistent_whitespace(context)

    # Then
    assert_no_output(result)
    assert_no_errors(result)
    assert_command_successful(result)


@then('all files are inconsistent.')
def assert_all_files_inconsistent(context):
    # When
    result = execute_consistent_whitespace(context)

    # Then
    assert_command_unsuccessful(result)
