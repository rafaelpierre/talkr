load:
	pyenv shell 3.11 && python -m venv .venv && source .venv/bin/activate && pip install locust && python tests/locust/basic.py