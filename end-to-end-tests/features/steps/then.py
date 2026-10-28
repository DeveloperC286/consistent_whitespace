from assertions import (
    assert_command_successful,
    assert_command_unsuccessful,
    assert_no_errors,
    assert_no_output,
)
from behave import then
from utilities import execute_consistent_whitespace, get_inconsistent_files


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


@then('all files are consistent across the codebase.')
def assert_all_files_consistent_across_codebase(context):
    # When
    result = execute_consistent_whitespace(context)

    # Then
    assert_no_output(result)
    assert_no_errors(result)
    assert_command_successful(result)


@then('files are inconsistent across the codebase.')
def assert_all_files_inconsistent_across_codebase(context):
    # When
    result = execute_consistent_whitespace(context)

    # Then
    assert_command_unsuccessful(result)


@then('only "{path}" is inconsistent across the codebase.')
def assert_only_file_inconsistent_across_codebase(context, path):
    # When
    inconsistent_files = get_inconsistent_files(context)

    # Then
    assert inconsistent_files == {path}, f"Expected only '{path}' to be inconsistent.\n" + \
        f"Inconsistent files = {sorted(inconsistent_files)}.\n"


@then('"{paths}" are inconsistent across the codebase.')
def assert_files_inconsistent_across_codebase(context, paths):
    # When
    inconsistent_files = get_inconsistent_files(context)

    # Then
    expected_files = set(paths.split(","))
    assert inconsistent_files == expected_files, f"Expected {sorted(expected_files)} to be inconsistent.\n" + \
        f"Inconsistent files = {sorted(inconsistent_files)}.\n"


@then('the output is in the github format.')
def assert_output_is_github_format(context):
    # When
    result = execute_consistent_whitespace(context)

    # Then
    assert_command_unsuccessful(result)
    assert "::error" in result.stdout, "Expected standard output to contain a GitHub Actions error annotation.\n" + \
        f"Standard output = {result.stdout.encode()}.\n"


@then('the output is in the pretty format.')
def assert_output_is_pretty_format(context):
    # When
    result = execute_consistent_whitespace(context)

    # Then
    assert_command_unsuccessful(result)
    assert result.stdout != "", "Expected standard output to be non-empty.\n"
    assert "::error" not in result.stdout, "Expected standard output to not contain a GitHub Actions error annotation.\n" + \
        f"Standard output = {result.stdout.encode()}.\n"


@then('the output is not empty.')
def assert_output_is_not_empty(context):
    # When
    result = execute_consistent_whitespace(context)

    # Then
    assert_command_unsuccessful(result)
    assert result.stdout != "", "Expected standard output to be non-empty.\n"


@then('the output is empty.')
def assert_output_is_empty(context):
    # When
    result = execute_consistent_whitespace(context)

    # Then
    assert_command_unsuccessful(result)
    assert_no_output(result)
