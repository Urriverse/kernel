_:
	@ 													\
	export PATH=$$PATH:$$PWD/etc/bin; 					\
	python3 etc/bin/_select_profile.py; 				\
	chmod +x etc/bin/do;								\
	if echo "$$SHELL" | grep -e "fish"; then			\
		$$SHELL -i -C 'source etc/bin/activate.fish';	\
	else 												\
		$$SHELL etc/bin/activate.sh; 					\
	fi
