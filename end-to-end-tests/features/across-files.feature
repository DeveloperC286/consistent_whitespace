Feature: Ensure consistent whitespace can be checked across all files.


  Scenario:
    Given an empty directory.
    When "two-spaces-indented.txt" is copied into the directory.
    When "two-spaces-indented.txt" is copied into the directory as "another-file.txt".
    Then all files are consistent across the codebase.


  Scenario:
    Given an empty directory.
    When "tab-indented.txt" is copied into the directory.
    When "tab-indented.txt" is copied into the directory as "another-file.txt".
    Then all files are consistent across the codebase.


  Scenario:
    Given an empty directory.
    When "two-spaces-indented.txt" is copied into the directory.
    When "tab-indented.txt" is copied into the directory.
    Then all files are inconsistent across the codebase.