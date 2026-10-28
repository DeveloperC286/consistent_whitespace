Feature: You can provide the paths of files/directories to check via standard input, using the path "-".


  Scenario:
    Given an empty directory.
    When "two-spaces-indented.txt" is copied to "file1.txt".
    And "mixed-within-line-indented.txt" is copied to "file2.txt".
    Then all files are inconsistent.
    When the paths "file1.txt" are provided via standard input.
    Then all files are consistent.


  Scenario:
    Given an empty directory.
    When "two-spaces-indented.txt" is copied to "file1.txt".
    And "mixed-within-line-indented.txt" is copied to "file2.txt".
    And the paths "file1.txt,file2.txt" are provided via standard input.
    Then all files are inconsistent.


  Scenario:
    Given an empty directory.
    When "two-spaces-indented.txt" is copied to "src/file1.txt".
    And "two-spaces-indented.txt" is copied to "src/file2.txt".
    And "mixed-within-line-indented.txt" is copied to "file.txt".
    Then all files are inconsistent.
    When the paths "src/" are provided via standard input.
    Then all files are consistent.


  Scenario:
    Given an empty directory.
    When "two-spaces-indented.txt" is copied to "src/file1.txt".
    And "two-spaces-indented.txt" is copied to "file1.txt".
    And "mixed-within-line-indented.txt" is copied to "file2.txt".
    Then all files are inconsistent.
    When the path "src/" is provided.
    And the paths "file1.txt" are provided via standard input.
    Then all files are consistent.
