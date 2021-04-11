requirements_done: requirements.txt
	pip3 install -r requirements.txt && touch requirements_done

requirements.txt: requirements.in
	pip-compile requirements.in > requirements.txt
