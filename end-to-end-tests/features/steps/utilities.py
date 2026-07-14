import os
from subprocess import Popen, PIPE


def execute_command(command, environment):
    process = Popen(
        command,
        shell=True,
        stdin=PIPE,
        stdout=PIPE,
        stderr=PIPE,
        env=environment)
    process.wait()

    result = type("Result", (), {})
    result.exit_code = process.returncode

    stdout, stderr = process.communicate()
    result.stdout = stdout.decode("utf-8")
    result.stderr = stderr.decode("utf-8")

    return result


def execute_consistent_whitespace(context):
    os.chdir(context.execution_directory)
    environment = os.environ.copy()
    environment.update(getattr(context, "environment_overrides", {}))
    result = execute_command(
        context.consistent_whitespace_path + " " + context.arguments, environment)
    os.chdir(context.behave_directory)
    return result
