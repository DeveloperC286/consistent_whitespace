Feature: You can provide the paths of files/directories to check, instead of the current directory.


  Scenario:
    Given an empty directory.
    When "two-spaces-indented.txt" is copied to "file1.txt".
    And "mixed-within-line-indented.txt" is copied to "file2.txt".
    Then all files are inconsistent.
    When the path "file1.txt" is provided.
    Then all files are consistent.


  Scenario:
    Given an empty directory.
    When "two-spaces-indented.txt" is copied to "src/file1.txt".
    And "two-spaces-indented.txt" is copied to "src/file2.txt".
    And "mixed-within-line-indented.txt" is copied to "file.txt".
    Then all files are inconsistent.
    When the path "src/" is provided.
    Then all files are consistent.
