import tempfile
import os


from behave import given


@given('an empty directory.')
def empty_directory(context):
    context.behave_directory = os.getcwd()
    context.execution_directory = tempfile.mkdtemp()
    context.consistent_whitespace_path = f"{context.behave_directory}/../target/debug/consistent_whitespace"  # fmt: off
