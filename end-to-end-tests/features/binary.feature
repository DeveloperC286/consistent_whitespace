Feature: Binary/non-UTF-8 files are skipped instead of aborting the run.


  Scenario:
    Given an empty directory.
    When "binary.png" is copied to "image.png".
    Then all files are consistent.


  Scenario:
    Given an empty directory.
    When "binary.png" is copied to "image.png".
    And "two-spaces-indented.txt" is copied to "file.txt".
    Then all files are consistent.


  Scenario:
    Given an empty directory.
    When "binary.png" is copied to "image.png".
    And "mixed-within-line-indented.txt" is copied to "file.txt".
    Then all files are inconsistent.
