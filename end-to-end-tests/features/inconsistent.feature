Feature: Ensure unsuccessful execution for inconsistent files.


  Scenario:
    Given an empty directory.
    When "mixed-within-line-indented.txt" is copied into the directory.
    Then all files are inconsistent.


  Scenario:
    Given an empty directory.
    When "mixed-across-lines-indented.txt" is copied into the directory.
    Then all files are inconsistent.
