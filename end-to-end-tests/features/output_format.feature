Feature: The output format can be forced regardless of the environment.


  Scenario:
    Given an empty directory.
    When "mixed-within-line-indented.txt" is copied to "file.txt".
    And the output format "github" is provided.
    Then the output is in the GitHub Actions format.


  Scenario:
    Given an empty directory.
    When "mixed-within-line-indented.txt" is copied to "file.txt".
    And the output format "pretty" is provided.
    Then the output is pretty.


  Scenario:
    Given an empty directory.
    When "mixed-within-line-indented.txt" is copied to "file.txt".
    And the output format "quiet" is provided.
    Then the output is empty.


  Scenario:
    Given an empty directory.
    When "mixed-within-line-indented.txt" is copied to "file.txt".
    And the GITHUB_ACTIONS environment variable is set.
    And the output format "pretty" is provided.
    Then the output is pretty.
