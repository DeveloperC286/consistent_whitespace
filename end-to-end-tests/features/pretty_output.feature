Feature: The pretty output format can be forced.


  Scenario:
    Given an empty directory.
    When "mixed-within-line-indented.txt" is copied to "file.txt".
    And the argument --output is provided as "pretty".
    Then the output is in the pretty format.


  Scenario:
    Given an empty directory.
    When "mixed-within-line-indented.txt" is copied to "file.txt".
    And the GITHUB_ACTIONS environment variable is set.
    And the argument --output is provided as "pretty".
    Then the output is in the pretty format.
