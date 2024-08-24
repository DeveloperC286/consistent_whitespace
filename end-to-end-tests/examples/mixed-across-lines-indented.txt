VERSION 0.6


clean-git-history-checking:
    FROM rust
    RUN cargo install clean_git_history
	COPY ".git" "."
	ARG from="origin/HEAD"
	RUN /usr/local/cargo/bin/clean_git_history --from-reference "${from}"


conventional-commits-linting:
    FROM rust
    RUN cargo install conventional_commits_linter
    COPY ".git" "."
    ARG from="origin/HEAD"
    RUN /usr/local/cargo/bin/conventional_commits_linter --from-reference "${from}" --allow-angular-type-only


formatting-base:
    FROM python:3-slim
    RUN pip3 install "autopep8"
    COPY "*.py" "."


check-formatting:
    FROM +formatting-base
    RUN find "." -maxdepth 1 -type f -name "*.py" | xargs -I {} autopep8 --exit-code --diff --aggressive --aggressive "{}"


fix-formatting:
    FROM +formatting-base
    RUN find "." -maxdepth 1 -type f -name "*.py" | xargs -I {} autopep8 --in-place --aggressive --aggressive "{}"
    SAVE ARTIFACT "*.py" AS LOCAL "./"


python3-base:
    FROM python:3-slim
    COPY "requirements.txt" "."
    RUN pip3 install -r "requirements.txt"
    COPY "*.py" "."


compiling:
    FROM +python3-base
    RUN find "." -maxdepth 1 -type f -name "*.py" | xargs -I {} python3 -m py_compile "{}"
