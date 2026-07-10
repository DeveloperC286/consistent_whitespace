Feature: Ensure consistent whitespace can be checked across all files.


  Scenario:
    Given an empty directory.
    When "two-spaces-indented.txt" is copied to "file1.txt".
    And "two-spaces-indented.txt" is copied to "file2.txt".
    Then all files are consistent across the codebase.


  Scenario:
    Given an empty directory.
    When "tab-indented.txt" is copied to "file1.txt".
    And "tab-indented.txt" is copied to "file2.txt".
    Then all files are consistent across the codebase.


  Scenario:
    Given an empty directory.
    When "two-spaces-indented.txt" is copied to "file1.txt".
    And "tab-indented.txt" is copied to "file2.txt".
    Then files are inconsistent across the codebase.
