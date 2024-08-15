Feature: Ensure unsuccessful execution for inconsistent files.


  Scenario:
    Given an empty directory.
    When a mix indented file is copied into the directory.
	Then all files are inconsistent.
