Feature: The default output format is automatically detected based on the environment.


  Scenario:
    Given an empty directory.
    When "mixed-within-line-indented.txt" is copied to "file.txt".
    And the GITHUB_ACTIONS environment variable is set.
    Then the output is in the GitHub Actions format.


  Scenario:
    Given an empty directory.
    When "mixed-within-line-indented.txt" is copied to "file.txt".
    Then the output is pretty.
