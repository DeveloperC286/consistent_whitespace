Feature: The github output format can be forced.


  Scenario:
    Given an empty directory.
    When "mixed-within-line-indented.txt" is copied to "file.txt".
    And the argument --output is provided as "github".
    Then the output is in the github format.
