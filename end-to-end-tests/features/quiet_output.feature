Feature: The quiet output format can be forced.


  Scenario:
    Given an empty directory.
    When "mixed-within-line-indented.txt" is copied to "file.txt".
    And the argument --output is provided as "quiet".
    Then the output is in the quiet format.
