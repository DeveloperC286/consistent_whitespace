Feature: Ensure consistent whitespace can be checked across all files.


  Scenario:
    Given an empty directory.
    When "two-spaces-indented.txt" is copied to "file1.txt".
    And "two-spaces-indented.txt" is copied to "file2.txt".
    Then all files are consistent across the codebase.


  Scenario:
    Given an empty directory.
    When "tab-indented.txt" is copied to "file1.txt".
    And "tab-indented.txt" is copied to "file2.txt".
    Then all files are consistent across the codebase.


  Scenario:
    Given an empty directory.
    When "two-spaces-indented.txt" is copied to "file1.txt".
    And "tab-indented.txt" is copied to "file2.txt".
    Then files are inconsistent across the codebase.


  Scenario: The majority whitespace is expected, regardless of path order.
    Given an empty directory.
    When "tab-indented.txt" is copied to "a.txt".
    And "two-spaces-indented.txt" is copied to "b.txt".
    And "two-spaces-indented.txt" is copied to "c.txt".
    And "two-spaces-indented.txt" is copied to "d.txt".
    And "two-spaces-indented.txt" is copied to "e.txt".
    Then only "a.txt" is inconsistent across the codebase.


  Scenario: The majority whitespace is expected, regardless of path order.
    Given an empty directory.
    When "two-spaces-indented.txt" is copied to "a.txt".
    And "two-spaces-indented.txt" is copied to "b.txt".
    And "two-spaces-indented.txt" is copied to "c.txt".
    And "two-spaces-indented.txt" is copied to "d.txt".
    And "tab-indented.txt" is copied to "e.txt".
    Then only "e.txt" is inconsistent across the codebase.


  Scenario: When there is no majority whitespace, all files are inconsistent.
    Given an empty directory.
    When "tab-indented.txt" is copied to "a.txt".
    And "tab-indented.txt" is copied to "b.txt".
    And "two-spaces-indented.txt" is copied to "c.txt".
    And "two-spaces-indented.txt" is copied to "d.txt".
    Then "a.txt,b.txt,c.txt,d.txt" are inconsistent across the codebase.
