def assert_command_successful(result):
    assert result.exit_code == 0, "Expected a zero exit code to indicate a successful execution.\n" + \
        f"Exit code = '{result.exit_code}'.\n"


def assert_command_unsuccessful(result):
    assert result.exit_code != 0, "Expected a non-zero exit code to indicate a unsuccessful execution\n" + \
        f"Exit code = '{result.exit_code}'.\n"


def assert_no_output(result):
    assert result.stdout == "", "Expected standard output to be empty.\n" + \
        f"Standard output = {result.stdout.encode()}.\n"


def assert_no_errors(result):
    assert result.stderr == "", "Expected standard error to be empty.\n" + \
        f"Standard error = {result.stderr.encode()}.\n"
