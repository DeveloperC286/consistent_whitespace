Feature: Ensure successful execution for consistent files.


  Scenario:
    Given an empty directory.
	Then all files are consistent.


  Scenario:
    Given an empty directory.
    When a two space indented file is copied into the directory.
	Then all files are consistent.


  Scenario:
    Given an empty directory.
    When a tab indented file is copied into the directory.
	Then all files are consistent.
