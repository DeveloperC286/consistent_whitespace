import os
import re
from subprocess import PIPE, Popen

# Environment variables which change consistent_whitespace's behaviour, they are scrubbed from the inherited environment
# so scenarios are isolated from the environment behave itself is running in (e.g. GitHub Actions).
SCRUBBED_ENVIRONMENT_VARIABLES = ["GITHUB_ACTIONS", "RUST_LOG", "RUST_BACKTRACE", "RUST_LIB_BACKTRACE"]


def execute_command(command, cwd=None, environment_overrides=None, stdin=None):
    environment = os.environ.copy()

    for variable in SCRUBBED_ENVIRONMENT_VARIABLES:
        environment.pop(variable, None)

    environment.update(environment_overrides or {})

    process = Popen(
        command,
        shell=True,
        stdin=PIPE,
        stdout=PIPE,
        stderr=PIPE,
        cwd=cwd,
        env=environment)

    stdout, stderr = process.communicate(input=(stdin or "").encode("utf-8"))

    result = type("Result", (), {})
    result.exit_code = process.returncode
    result.stdout = stdout.decode("utf-8")
    result.stderr = stderr.decode("utf-8")

    return result


def execute_consistent_whitespace(context):
    return execute_command(
        f"{context.consistent_whitespace_path} {context.arguments}",
        cwd=context.execution_directory,
        environment_overrides=context.environment_overrides,
        stdin=getattr(context, "stdin", ""))


def get_inconsistent_files(context):
    result = execute_command(
        f"{context.consistent_whitespace_path} {context.arguments} --output github",
        cwd=context.execution_directory,
        environment_overrides=context.environment_overrides,
        stdin=getattr(context, "stdin", ""))

    assert result.exit_code != 0, "Expected a non-zero exit code to indicate a unsuccessful execution\n" + \
        f"Exit code = '{result.exit_code}'.\n"

    return {os.path.normpath(path) for path in re.findall(r"^::error file=([^,]+?)::", result.stdout, re.MULTILINE)}
