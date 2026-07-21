import shutil
import os

from behave import when


@when('"{filename}" is copied to "{path}".')
def file_copied_to_path(context, filename, path):
    source_path = context.behave_directory + f"/examples/{filename}"
    destination_path = context.execution_directory + f"/{path}"
    os.makedirs(os.path.dirname(destination_path), exist_ok=True)
    shutil.copy2(source_path, destination_path)


@when('the path "{path}" is provided.')
def path_provided(context, path):
    context.arguments = f" {path} "


@when('the whitespace "{whitespace}" is provided.')
def whitespace_provided(context, whitespace):
    context.arguments = f" --whitespace {whitespace} "


@when('the output format "{output_format}" is provided.')
def output_format_provided(context, output_format):
    context.arguments += f" --output {output_format} "


@when('the GITHUB_ACTIONS environment variable is set.')
def github_actions_environment_variable_set(context):
    context.environment_overrides["GITHUB_ACTIONS"] = "true"
