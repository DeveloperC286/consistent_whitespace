Feature: The quiet output format can be forced.


  Scenario:
    Given an empty directory.
    When "mixed-within-line-indented.txt" is copied to "file.txt".
    Then the output is not empty.
    Given the arguments are reset.
    When the argument --output is provided as "quiet".
    Then the output is empty.
