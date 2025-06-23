Feature: Ensure successful execution for consistent files.


  Scenario:
    Given an empty directory.
    Then all files are consistent.


  Scenario:
    Given an empty directory.
    When "two-spaces-indented.txt" is copied into the directory.
    Then all files are consistent.


  Scenario:
    Given an empty directory.
    When "tab-indented.txt" is copied into the directory.
    Then all files are consistent.