FROM python:3.13.6-alpine3.21@sha256:5bcbb2a02505d35034426faf87c647adfa7b42c275a21ef0b5014350e99991f0
RUN apk add --no-cache \
	py3-autopep8=2.1.0-r1

ENTRYPOINT ["autopep8", "--in-place", "--aggressive", "--aggressive", "--max-line-length", "120", "--recursive"]
CMD ["."]
