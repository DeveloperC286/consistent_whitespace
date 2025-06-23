Feature: Ensure you can provide the only whitespace you will accept.


  Scenario:
    Given an empty directory.
    When a two space indented file is copied into the directory.
    Then all files are consistent.
    When the whitespace "tabs" is provided.
    Then all files are inconsistent.


  Scenario:
    Given an empty directory.
    When a tab indented file is copied into the directory.
    Then all files are consistent.
    When the whitespace "spaces" is provided.
    Then all files are inconsistent.