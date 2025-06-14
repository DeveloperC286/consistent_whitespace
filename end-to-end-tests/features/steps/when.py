import shutil

from behave import when


@when('a two space indented file is copied into the directory.')
def two_space_indeted_file_copied(context):
    shutil.copy2(context.behave_directory +
                 "/examples/two-spaces-indented.txt", context.execution_directory)


@when('a tab indented file is copied into the directory.')
def two_space_indeted_file_copied(context):
    shutil.copy2(context.behave_directory +
                 "/examples/tab-indented.txt", context.execution_directory)


@when('a mixed within line indented file is copied into the directory.')
def two_space_indeted_file_copied(context):
    shutil.copy2(context.behave_directory +
                 "/examples/mixed-within-line-indented.txt", context.execution_directory)


@when('a mixed across lines indented file is copied into the directory.')
def two_space_indeted_file_copied(context):
    shutil.copy2(context.behave_directory +
                 "/examples/mixed-across-lines-indented.txt", context.execution_directory)


@when('the path "two-space-indented.txt" is provided.')
def two_space_indented_path_provided(context):
    context.arguments = " two-spaces-indented.txt "
