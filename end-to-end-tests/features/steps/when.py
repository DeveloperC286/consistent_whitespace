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


@when('a mix indented file is copied into the directory.')
def two_space_indeted_file_copied(context):
    shutil.copy2(context.behave_directory +
                 "/examples/mix-indented.txt", context.execution_directory)
