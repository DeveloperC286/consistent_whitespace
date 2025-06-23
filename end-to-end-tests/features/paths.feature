Feature: You can provide the paths of files/directories to check, instead of the current directory.


  Scenario:
    Given an empty directory.
    When "two-spaces-indented.txt" is copied into the directory.
    And "mixed-within-line-indented.txt" is copied into the directory.
    Then all files are inconsistent.
    When the path "two-spaces-indented.txt" is provided.
    Then all files are consistent.


  Scenario:
    Given an empty directory.
    When "two-spaces-indented.txt" is copied to "src/file1.txt".
    When "two-spaces-indented.txt" is copied to "src/file2.txt".
    And "mixed-within-line-indented.txt" is copied into the directory.
    Then all files are inconsistent.
    When the path "src/" is provided.
    Then all files are consistent.