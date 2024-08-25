Feature: Ensure unsuccessful execution for inconsistent files.


  Scenario:
    Given an empty directory.
    When a mixed within line indented file is copied into the directory.
    Then all files are inconsistent.


  Scenario:
    Given an empty directory.
    When a mixed across lines indented file is copied into the directory.
    Then all files are inconsistent.
